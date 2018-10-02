#![feature(try_from)]
#![recursion_limit = "500"]

mod chart;
mod ws;
mod data;

use self::chart::Model as Chart;
use yew::prelude::*;

fn main() {
    yew::initialize();
    let app: App<Chart> = App::new();
    let mut to_chart = app.mount_to_body();
    to_chart.send_message(chart::Msg::Init);
    yew::run_loop();
}
