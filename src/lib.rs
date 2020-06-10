pub mod config;
pub mod prelude;
pub mod tme;
pub mod utils;

use futures::prelude::*;
use tokio::net::TcpListener;
use tokio_serde::formats::SymmetricalBincode;
use tokio_util::codec::{FramedRead, LengthDelimitedCodec};

use embercore::*;

use crate::config::Config;

pub async fn run(settings: Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind(settings.server_address).await.unwrap();

    log::info!("listening on {:?}", listener.local_addr());

    let mut s = listener.incoming();

    while let Some(socket) = s.try_next().await.unwrap() {
        let length_delimited = FramedRead::new(socket, LengthDelimitedCodec::new());

        let mut deserialized = tokio_serde::SymmetricallyFramed::new(
            length_delimited,
            SymmetricalBincode::<protocol::Message>::default(),
        );

        tokio::spawn(async move {
            while let Some(msg) = deserialized.try_next().await.unwrap() {
                log::info!("GOT: {:?}", msg);
            }
        });
    }

    Ok(())
}
