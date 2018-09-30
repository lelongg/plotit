use std::{thread, time};

fn main() {
    let step = 0.1;
    let mut x = 0.0f64;
    let period = time::Duration::from_millis(30);
    thread::sleep(period * 10);
    loop {
        x += step;
        println!("{}", x.sin());
        thread::sleep(period);
    }
}