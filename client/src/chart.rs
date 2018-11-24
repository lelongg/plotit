use crate::data::Data;
use crate::ws::Model as WebSocket;
use crossbeam::{channel, Receiver, Sender};
use std::time::Duration;
use stdweb::traits::*;
use stdweb::web::document;
use yew::prelude::*;
use yew::services::interval::IntervalService;
use yew::services::Task;
use yew::*;

pub struct Model {
    chart: Option<stdweb::web::Element>,
    data_tx: Sender<Data>,
    data_rx: Receiver<Data>,
    state: State,
    _standalone: Option<Box<Task>>,
}

#[derive(Debug, PartialEq, Clone)]
enum State {
    Stopped,
    Running,
    Paused,
}

#[allow(dead_code)]
#[allow(unused_attributes)]
pub enum Msg {
    Init,
    Pause,
    Resume,
    Stop,
    AppendData(Data),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Props {}

impl Default for Props {
    fn default() -> Self {
        Props {}
    }
}

mod plotly {
    use stdweb::*;

    pub fn new_trace(x: &Vec<f64>, y: &Vec<f64>) -> Value {
        return js! {
            return {
                type: "scattergl",
                x: @{x},
                y: @{y},
            }
        };
    }

    pub fn add_new_trace(chart: &web::Element) {
        js! {
            Plotly.addTraces(@{chart}, [@{new_trace(&Vec::new(), &Vec::new())}]);
        };
    }

    pub fn new_plot(chart: &web::Element) {
        js! {
            let layout = @{new_layout()};

            Plotly.newPlot(@{chart}, [], layout, {
                responsive: true,
                displaylogo: false,
                scrollZoom: true,
            });
        };
    }

    pub fn new_layout() -> Value {
        return js! {
            return {
                dragmode: "pan",
                hovermode: "closest",
                xaxis: {
                    rangeslider: {
                        visible: false,
                    },
                },
                yaxis: {
                    fixedrange: false,
                }
            };
        };
    }

    pub fn extend_traces(chart: &web::Element, x: &Vec<Vec<f64>>, y: &Vec<Vec<f64>>) {
        let index_array: Vec<u32> = (0..x.len() as u32).collect();

        js! {
            Plotly.extendTraces(@{chart}, {x: @{x}, y: @{y}}, @{index_array});
        }
    }

    pub fn data_length(chart: &web::Element) -> usize {
        use stdweb::unstable::TryInto;
        let value = js! { return @{chart}.data.length };
        value.try_into().unwrap()
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let (data_tx, data_rx) = channel::unbounded();

        Model {
            chart: None,
            data_tx,
            data_rx,
            state: State::Stopped,
            _standalone: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Init => {
                let chart = document()
                    .query_selector("#chart")
                    .expect("cannot get chart element")
                    .expect("cannot unwrap chart element");

                self.chart = Some(chart.clone());
                let data_rx = self.data_rx.clone();

                plotly::new_plot(&chart);

                let callback = move |_| {
                    let length = data_rx.len();
                    let mut x = vec![Vec::with_capacity(length); 1];
                    let mut y = vec![Vec::with_capacity(length); 1];
                    for _ in 0..length {
                        if let Some(data) = data_rx.recv() {
                            x.resize(x.len().max(data.values.len()), Vec::new());
                            y.resize(y.len().max(data.values.len()), Vec::new());
                            for (index, value) in data.values.iter().enumerate() {
                                x[index].push(data.stamp);
                                y[index].push(value.clone());
                            }
                        }
                    }

                    if x.len() == 0 {
                        return;
                    };

                    let trace_count = plotly::data_length(&chart);
                    let trace_to_add = x.len().saturating_sub(trace_count);
                    for _ in 0..trace_to_add {
                        plotly::add_new_trace(&chart);
                    }
                    plotly::extend_traces(&chart, &x, &y);
                };

                let mut interval = IntervalService::new();
                self._standalone = Some(Box::new(
                    interval.spawn(Duration::from_millis(100), callback.into()),
                ));

                self.state = State::Running;
                return true;
            }
            Msg::AppendData(data) => {
                if self.state == State::Running {
                    self.data_tx.send(data);
                }
            }
            Msg::Pause => {
                self.state = State::Paused;

                if let Some(ref chart) = self.chart {
                    plotly::extend_traces(chart, &Vec::new(), &Vec::new());
                }

                return true;
            }
            Msg::Resume => {
                self.state = State::Running;
                return true;
            }
            Msg::Stop => {
                self.state = State::Stopped;
                return true;
            }
        }
        false
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let state = self.state.clone();
        let icon_class = match self.state {
            State::Paused => "zmdi zmdi-play",
            _ => "zmdi zmdi-pause",
        };

        html! {
            <div id="chart", style="position: relative; height:100vh; width:100%",/>
            <WebSocket<Data>:
                url="ws://127.0.0.1:9001",
                ondata=|data| Msg::AppendData(data),
                ondisconnect=|_| Msg::Stop,/>
            <button
                class="fab fab--material",
                style="position: absolute; right: 1rem; bottom: 1rem; cursor: pointer;",
                disabled={state == State::Stopped},
                onclick=|_| {
                    match state {
                        State::Paused => Msg::Resume,
                        _ => Msg::Pause,
                    }
                },>
                <i class={icon_class},></i>
            </button>
        }
    }
}
