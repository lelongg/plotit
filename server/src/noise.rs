use std::{thread, time};

fn main() {
    let step = 0.01;
    let mut x = 0.0f64;
    let period = time::Duration::from_millis(10);
    thread::sleep(period * 10);
    loop {
        x += step;
        println!("{}, {}, {}, {}", x.sin(), x.cos(), (x*x).sin().cos(), (2.0*x).exp().sin());
        thread::sleep(period);
    }
}