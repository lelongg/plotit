use crate::data::Data;
use crate::ws::Model as WebSocket;
use stdweb::traits::*;
use stdweb::web::document;
use stdweb::*;
use yew::prelude::*;
use yew::services::websocket::WebSocketService;
use yew::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Model {
    chart: Option<stdweb::web::Element>,
    x_min: Option<f64>,
    x_max: Option<f64>,
    state: State,
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

impl<CTX> Component<CTX> for Model
where
    CTX: AsMut<WebSocketService> + 'static,
{
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Model {
            chart: None,
            x_min: None,
            x_max: None,
            state: State::Stopped,
        }
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::Init => {
                self.chart = Some(document()
                    .query_selector("#chart")
                    .expect("cannot get chart element")
                    .expect("cannot unwrap chart element"));
                
                js! {
                    var trace = {
                        type: "line",
                        // mode: "lines+markers",
                        x: [],
                        y: [],
                        marker: {
                            color: "#C8A2C8",
                            line: {
                                width: 2.5
                            }
                        }
                    };

                    var data = [ trace ];

                    var layout = {
                        title: "Simple example",
                        font: {size: 18},
                        dragmode: "pan",
                        hovermode: "closest",
                        xaxis: {
                            rangeslider: {}
                        },
                    };

                    Plotly.newPlot(@{&self.chart}, data, layout, {
                        responsive: true,
                        displaylogo: false,
                        scrollZoom: true,});
                };

                self.state = State::Running;
                return true;
            }
            Msg::AppendData(data) => {
                if self.state == State::Running {
                    self.x_min = Some(self.x_min.map_or(data.stamp, |v| v.min(data.stamp)));
                    self.x_max = Some(self.x_max.map_or(data.stamp, |v| v.max(data.stamp)));

                    if let Some(ref chart) = self.chart {
                        js! {
                            Plotly.extendTraces(
                                @{chart}, {x: [[@{data.stamp}]], y: [[@{data.value}]]}, [0])
                        }
                    }
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
            Msg::Stop => (),
        }
        false
    }
}

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: AsMut<WebSocketService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        let state = self.state.clone();

        html! {
            <div id="chart", style="position: relative; height:95vh; width:100%",/>
            <WebSocket<Data>: ondata=|data| Msg::AppendData(data),/>
            <button onclick=|_| {
                match state {
                    State::Paused => Msg::Resume,
                    State::Running => Msg::Pause,
                    _ => Msg::Pause,
                }
            },>{
                match self.state {
                    State::Paused => "Resume",
                    State::Running => "Pause",
                    _ => "Pause",
                }
            }</button>
        }
    }
}
