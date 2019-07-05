#![feature(duration_float)]

#[macro_use]
extern crate futures;

mod config;
mod codec;
mod client;
mod state;
mod protocol;

use std::error::Error;
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

use tokio::prelude::*;
use tokio::net::{TcpListener, TcpStream};
use tokio::net::tcp::Incoming;
use tokio::timer::Interval;
use futures::future;
use futures::sync::mpsc;

use crate::state::{Shared, State};
use crate::client::{Peer, ResponsesTx};
use crate::config::Config;
use std::time::Instant;

pub fn run() -> Result<(), Box<dyn Error>> {
    let config = Config::new()?;

    let shared = Arc::new(Mutex::new(Shared::new()));

    let (response_tx, response_rx) = mpsc::unbounded();

    let state = State::new(shared.clone(), response_rx);

    let listener = create_listener(&config)?
        .map_err(|e| eprintln!("Accept failed: {:?}", e))
        .for_each(move |socket| {
            process_connection(socket, shared.clone(), response_tx.clone());
            Ok(())
        });

    let mut current_time = Instant::now();
    let mut last_time = current_time;

    let game_loop = Interval::new_interval(std::time::Duration::from_millis(20))
        .fold(state, move |mut state, _| {
            current_time = Instant::now();
            let dt = current_time - last_time;
            last_time = current_time;

            state.game_loop(dt.as_secs_f64());
            future::ok(state)
        })
        .map(|_| {})
        .map_err(|e| eprintln!("Game loop error: {:?}", e));

    let port = config.port;
    tokio::run(future::lazy(move || {
        println!("Listening on port {}", port);

        tokio::spawn(listener);

        tokio::spawn(game_loop);

        Ok(())
    }));

    Ok(())
}

fn create_listener(config: &Config) -> Result<Incoming, String> {
    let address = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = match TcpListener::bind(&address) {
        Ok(server) => server,
        Err(e) => return Err(e.to_string())
    };

    Ok(listener.incoming())
}


fn process_connection(socket: TcpStream, state: Arc<Mutex<Shared>>, responses_queue: ResponsesTx) {
    use crate::codec::*;

    let client = Peer::new(state, Codec::new(socket), responses_queue);

    let connection = client
        .map_err(|e| {
            println!("Message error: {:?}", e);
        });

    tokio::spawn(connection);
}
