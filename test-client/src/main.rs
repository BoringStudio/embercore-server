mod config;

use clap::{
    App,
    Arg,
};
use futures::SinkExt;
use tokio::net::TcpStream;
use tokio_serde::formats::SymmetricalBincode;
use tokio_util::codec::{
    FramedWrite,
    LengthDelimitedCodec,
};

use embercore::*;

use crate::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    embercore::utils::init_logger();

    let matches = App::new("test-client")
        .version("1.0")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let config = {
        let path = matches.value_of("config").unwrap();
        Config::new(path)?
    };

    let socket = TcpStream::connect(config.server_address).await.unwrap();

    let length_delimited = FramedWrite::new(socket, LengthDelimitedCodec::new());

    let mut serialized = tokio_serde::SymmetricallyFramed::new(
        length_delimited,
        SymmetricalBincode::<protocol::Message>::default(),
    );

    serialized
        .send(protocol::Message {
            timestamp: chrono::Utc::now().timestamp(),
            test_text: "".to_string(),
        })
        .await
        .unwrap();

    Ok(())
}
