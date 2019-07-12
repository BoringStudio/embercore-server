#![feature(duration_float)]

#[macro_use]
extern crate futures;
#[macro_use]
extern crate specs_derive;

use std::error::Error;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use futures::future;
use futures::sync::mpsc;
use tokio::net::tcp::Incoming;
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::timer::Interval;

use crate::config::Config;
use crate::connection::Connection;
use crate::peer::PeersContainer;
use crate::state::State;

mod game;
mod protocol;

mod config;
mod codec;
mod peer;
mod connection;
mod state;

pub fn run() -> Result<(), Box<dyn Error>> {
    let config = Config::new()?;

    let (connection_events_tx, connection_events_rx) = mpsc::unbounded();

    let peers_container = Arc::new(Mutex::new(PeersContainer::new(connection_events_tx)));

    let (response_tx, response_rx) = mpsc::unbounded();

    let state = State::new(peers_container.clone(), response_rx, connection_events_rx);

    let listener = create_listener(&config)?
        .map_err(|e| eprintln!("Accept failed: {:?}", e))
        .for_each(move |socket| {
            let connection = Connection::new(socket,
                                             response_tx.clone(), peers_container.clone())
                .map_err(|e| {
                    println!("Error: {:?}", e);
                });

            tokio::spawn(connection);
            Ok(())
        });

    let mut current_time = Instant::now();
    let mut last_time = current_time;

    let game_loop = Interval::new_interval(std::time::Duration::from_millis(20))
        .fold(state, move |mut state, _| {
            current_time = Instant::now();
            let dt = current_time - last_time;
            last_time = current_time;

            state.on_update(dt.as_secs_f64());
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

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
