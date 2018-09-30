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
    chart: Option<stdweb::Value>,
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
                let ctx = document()
                    .query_selector("#chart")
                    .expect("cannot get chart element")
                    .expect("cannot unwrap chart element");

                self.chart = Some(js! {
                    return new Chart(@{ctx}, {
                        type: "line",
                        data: {
                            datasets: [{
                                label: "sin(x)",
                                borderColor: "rgba(255, 0, 0, 1.0)",
                                data: [],
                                fill: false,
                            }]
                        },
                        options: {
                            responsive: true,
                            animation: {
                                duration: 0,
                            },
                            hover: {
                                animationDuration: 0,
                            },
                            responsiveAnimationDuration: 0,
                            title: {
                                display: true,
                                text: "Simple example"
                            },
                            tooltips: {
                                mode: "nearest",
                                intersect: false,
                                style: {pointerEvents: "none"},
                            },
                            hover: {
                                mode: "nearest",
                                intersect: false
                            },
                            scales: {
                                xAxes: [{
                                    type: "linear",
                                    position: "bottom",
                                }],
                                yAxes: [{
                                    display: true,
                                    scaleLabel: {
                                        display: true,
                                        labelString: "y"
                                    }
                                }]
                            },
                            pan: {
                                enabled: true,
                                mode: "xy"
                            },
                            zoom: {
                                enabled: true,
                                mode: "xy",
                                limits: {
                                    max: 10,
                                    min: 0.5
                                }
                            },
                            downsample: {
                                enabled: false,
                                threshold: 50,
                                auto: true,
                                onInit: true,
                                preferOriginalData: false,
                                restoreOriginalData: true,
                            },
                        }
                    });
                });

                self.state = State::Running;
                return true;
            }
            Msg::AppendData(data) => {
                if self.state == State::Running {
                    self.x_min = Some(self.x_min.map_or(data.stamp, |v| v.min(data.stamp)));
                    self.x_max = Some(self.x_max.map_or(data.stamp, |v| v.max(data.stamp)));

                    if let Some(ref chart) = self.chart {
                        js! {
                            let chart = @{chart};
                            chart.data.datasets.forEach((dataset) => {
                                dataset.data.push({
                                    x: @{data.stamp},
                                    y: @{data.value}
                                });
                            });
                            chart.update();
                        }
                    }
                }
            }
            Msg::Pause => {
                self.state = State::Paused;
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
            <div class="chart-container", style="position: relative; height:90vh; width:90vw",>
                <canvas id="chart",></canvas>
            </div>
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
