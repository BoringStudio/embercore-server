mod config;

use std::borrow::Cow;
use std::net::SocketAddr;

use future::lazy;
use tokio::prelude::*;
use tokio::net::TcpListener;
use tokio::net::tcp::Incoming;

use config::*;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(-1);
    }
}

fn run() -> Result<(), Cow<'static, str>> {
    let config = Config::new()?;

    let listener = create_listener(&config)?
        .map_err(|e| eprintln!("Accept failed: {:?}", e))
        .for_each(|socket| {
            let (reader, writer) = socket.split();

            // simple temp echo
            let reply_back = tokio::io::copy(reader, writer);
            let handle_connection = reply_back.map(|(count, _, _)| {
                println!("Replied {:?} bytes", count);
            }).map_err(|err| {
                println!("I/O error {}", err);
            });

            tokio::spawn(handle_connection)
        });

    let port = config.port;
    tokio::run(lazy(move || {
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
