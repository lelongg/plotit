use std::{thread, time};

fn main() {
    let step = 0.1;
    let mut x = 0.0f64;
    let period = time::Duration::from_millis(100);
    loop {
        thread::sleep(period);
        x += step;
        println!("{}, {}", x.sin(), x.cos());
    }
}