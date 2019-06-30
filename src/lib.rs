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
use futures::future;

use crate::state::Shared;
use crate::client::Peer;
use crate::config::Config;

pub fn run() -> Result<(), Box<dyn Error>> {
    let config = Config::new()?;

    let state = Arc::new(Mutex::new(Shared::new()));

    let listener = create_listener(&config)?
        .map_err(|e| eprintln!("Accept failed: {:?}", e))
        .for_each(move |socket| {
            process_connection(socket, state.clone());
            Ok(())
        });

    let port = config.port;
    tokio::run(future::lazy(move || {
        println!("Listening on port {}", port);

        tokio::spawn(listener);

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


fn process_connection(socket: TcpStream, state: Arc<Mutex<Shared>>) {
    use crate::codec::*;

    let client = Peer::new(state, Codec::new(socket));

    let connection = client
        .map_err(|e| {
            println!("Message error: {:?}", e);
        });

    tokio::spawn(connection);
}
