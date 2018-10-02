use crate::data::Data;
use crate::ws::Model as WebSocket;
use crossbeam::{channel, Receiver, Sender};
use std::time::Duration;
use stdweb::traits::*;
use stdweb::web::document;
use stdweb::*;
use yew::prelude::*;
use yew::services::interval::IntervalService;
use yew::services::Task;
use yew::*;

pub struct Model {
    chart: Option<stdweb::web::Element>,
    x_min: Option<f64>,
    x_max: Option<f64>,
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

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let (data_tx, data_rx) = channel::unbounded();

        Model {
            chart: None,
            x_min: None,
            x_max: None,
            data_tx,
            data_rx,
            state: State::Stopped,
            _standalone: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Init => {
                self.chart = Some(
                    document()
                        .query_selector("#chart")
                        .expect("cannot get chart element")
                        .expect("cannot unwrap chart element"),
                );

                js! {
                    let trace = {
                        type: "line",
                        x: [],
                        y: [],
                        marker: {
                            color: "#C8A2C8",
                            line: {
                                width: 2.5
                            }
                        }
                    };

                    let data = [ trace ];

                    let layout = {
                        dragmode: "pan",
                        hovermode: "closest",
                        xaxis: {
                            rangeslider: {}
                        },
                    };

                    Plotly.newPlot(@{&self.chart}, data, layout, {
                        responsive: true,
                        displaylogo: false,
                        scrollZoom: true,
                    });
                };

                let chart = self.chart.clone();
                let data_rx = self.data_rx.clone();

                let callback = move |_| {
                    let length = data_rx.len();
                    let mut x = Vec::with_capacity(length);
                    let mut y = Vec::with_capacity(length);
                    for _ in 0..length {
                        if let Some(data) = data_rx.recv() {
                            x.push(data.stamp);
                            y.push(data.value);
                        }
                    }

                    if x.len() == 0 {
                        return;
                    };

                    if let Some(ref chart) = chart {
                        js! {
                            Plotly.extendTraces(
                                @{chart}, {x: [@{x}], y: [@{y}]}, [0])
                        }
                    }
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
                    self.x_min = Some(self.x_min.map_or(data.stamp, |v| v.min(data.stamp)));
                    self.x_max = Some(self.x_max.map_or(data.stamp, |v| v.max(data.stamp)));
                    self.data_tx.send(data);
                }
            }
            Msg::Pause => {
                self.state = State::Paused;

                if let Some(ref chart) = self.chart {
                    js! {
                        Plotly.extendTraces(
                            @{chart}, {x: [[null]], y: [[null]]}, [0])
                    }
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
