#![feature(plugin)]
#![feature(try_from)]
#![plugin(rocket_codegen)]

use client::data::Data;
use crossbeam::{channel, select, Receiver, Sender};
use failure::Error;
use float_duration::FloatDuration;
use rocket::response::NamedFile;
use std::convert::TryInto;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::SystemTime;
use websocket::sync::Server;
use websocket::OwnedMessage;

type Result<T> = std::result::Result<T, Error>;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("./target/deploy/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("./target/deploy/").join(file)).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, files])
}

fn ws(input_rx: Receiver<String>, output_tx: Sender<String>) {
    let ws_host = "127.0.0.1:9001";
    let server = Server::bind(ws_host).expect(&format!("Could not bind to {}", ws_host));
    let start_time = SystemTime::now();

    println!("websocket server on {}", ws_host);

    thread::spawn(move || {
        for request in server.filter_map(std::result::Result::ok) {
            let input_rx = input_rx.clone();
            let output_tx = output_tx.clone();

            thread::spawn(move || {
                println!("New connection attempt");

                let client = request.accept().unwrap();

                let ip = client.peer_addr().unwrap();

                println!("Connection from {}", ip);

                let (mut ws_receiver, mut ws_sender) = client.split().unwrap();
                let (internal_tx, internal_rx) = channel::unbounded();

                let input_rx = input_rx.clone();

                thread::spawn(move || loop {
                    select! {
                        recv(input_rx, data) => {
                            if let (Ok(stamp), Some(value)) = (start_time.elapsed(), data) {
                                if let Ok(data) = (
                                    Data {
                                        stamp: FloatDuration::from_std(stamp).as_seconds(),
                                        value
                                    }).try_into() {
                                    ws_sender.send_message(&OwnedMessage::Text(data)).ok();
                                }
                            }
                        }
                        recv(internal_rx, message) => {
                            match message {
                                Some(message) => { ws_sender.send_message(&message).ok(); }
                                None => return,
                            }
                        }
                    }
                });

                for message in ws_receiver.incoming_messages() {
                    let message = message.unwrap();

                    match message {
                        OwnedMessage::Close(_) => {
                            println!("Client {} disconnected", ip);
                            return;
                        }
                        OwnedMessage::Ping(ping) => {
                            let message = OwnedMessage::Pong(ping);
                            internal_tx.send(message);
                        }
                        OwnedMessage::Text(data) => {
                            output_tx.send(data);
                        }
                        _ => {}
                    }
                }
            });
        }
    });
}

fn main() -> Result<()> {
    let (input_tx, input_rx) = channel::bounded(5);
    let (output_tx, _) = channel::bounded(1);

    thread::spawn(move || {
        rocket().launch();
    });

    ws(input_rx.clone(), output_tx.clone());

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    loop {
        let mut input = String::new();
        match handle.read_line(&mut input) {
            Ok(_) => {
                input_tx.send(input.trim_right().to_owned());
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
