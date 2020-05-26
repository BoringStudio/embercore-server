use futures::SinkExt;
use tokio::net::TcpStream;
use tokio_serde::formats::SymmetricalBincode;
use tokio_util::codec::{FramedWrite, LengthDelimitedCodec};

use models::*;

#[tokio::main]
async fn main() {
    let socket = TcpStream::connect("127.0.0.1:10101").await.unwrap();

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
}
