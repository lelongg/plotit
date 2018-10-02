#![feature(plugin)]
#![feature(try_from)]
#![feature(decl_macro)]
#![feature(proc_macro_non_items)]
#![plugin(rocket_codegen)]

use client::data::Data;
use crossbeam::{channel, select, Receiver, Sender};
use failure::Error;
use float_duration::FloatDuration;
use rocket::http::{ContentType, Status};
use rocket::response;
use rocket::*;
use rust_embed::RustEmbed;
use std::ffi::OsStr;
use std::io::Cursor;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::thread;
use std::time::SystemTime;
use websocket::sync::Server;
use websocket::OwnedMessage;

#[derive(RustEmbed)]
#[folder = "./target/deploy/"]
struct Asset;

type Result<T> = std::result::Result<T, Error>;

// fn parse_data(input: &str) -> Data {

// }

fn ws(input_rx: Receiver<String>, output_tx: Sender<String>) {
    let ws_host = "127.0.0.1:9001";
    let server = Server::bind(ws_host).expect(&format!("Could not bind to {}", ws_host));

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
                            if let Some(data) = data {
                                ws_sender.send_message(&OwnedMessage::Text(data)).ok();
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

#[get("/")]
fn index<'r>() -> response::Result<'r> {
    Asset::get("index.html").map_or_else(
        || Err(Status::NotFound),
        |d| {
            response::Response::build()
                .header(ContentType::HTML)
                .sized_body(Cursor::new(d))
                .ok()
        },
    )
}

#[get("/<file..>")]
fn dist<'r>(file: PathBuf) -> response::Result<'r> {
    let filename = file.display().to_string();
    let ext = file
        .as_path()
        .extension()
        .and_then(OsStr::to_str)
        .expect("Could not get file extension");
    let content_type = ContentType::from_extension(ext).expect("Could not get file content type");
    Asset::get(&filename.clone()).map_or_else(
        || Err(Status::NotFound),
        |d| {
            response::Response::build()
                .header(content_type)
                .sized_body(Cursor::new(d))
                .ok()
        },
    )
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, dist])
}

fn main() -> Result<()> {
    let start_time = SystemTime::now();
    let (input_tx, input_rx) = channel::bounded(5);
    let (output_tx, _) = channel::bounded(1);

    thread::spawn(move || {
        rocket().launch();
    });

    ws(input_rx.clone(), output_tx.clone());

    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .trim(csv::Trim::All)
        .from_reader(handle);

    for result in reader.records() {
        let stamp = start_time.elapsed()?;
        let stamp = FloatDuration::from_std(stamp).as_seconds();

        input_tx.send(
            Data {
                stamp,
                values: result?
                    .iter()
                    .map(|field| field.parse().unwrap(),
                    ).collect::<Vec<_>>(),
            }.into(),
        );
    }

    Ok(())
}
