#![feature(try_from)]
#![recursion_limit = "500"]

mod chart;
mod ws;
mod data;

use self::chart::Model as Chart;
use yew::prelude::*;
use yew::services::websocket::WebSocketService;

struct Context {
    ws: WebSocketService,
}

impl AsMut<WebSocketService> for Context {
    fn as_mut(&mut self) -> &mut WebSocketService {
        &mut self.ws
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        ws: WebSocketService::new(),
    };
    let app: App<Context, Chart> = App::new(context);
    let mut to_chart = app.mount_to_body();
    to_chart.send_message(chart::Msg::Init);
    yew::run_loop();
}
