use bytes::BytesMut;
use chrono::prelude::*;
use prost::{decode_length_delimiter, Message};
use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;

use crate::protocol::base::*;
use crate::protocol::base::general_message::Payload;

pub struct Codec {
    pub socket: TcpStream,
    pub input_buffer: BytesMut,
    pub output_buffer: BytesMut,

    pub input_message_size: Option<usize>,
}

impl Codec {
    pub fn new(socket: TcpStream) -> Self {
        Codec {
            socket,
            input_buffer: BytesMut::new(),
            output_buffer: BytesMut::new(),
            input_message_size: None,
        }
    }

    pub fn buffer(&mut self, payload: general_message::Payload) {
        let message = GeneralMessage {
            time: Utc::now().timestamp_millis(),
            payload: Some(payload),
        };

        message.encode_length_delimited(&mut self.output_buffer).unwrap();
    }

    pub fn poll_flush(&mut self) -> Poll<(), io::Error> {
        while !self.output_buffer.is_empty() {
            let n = try_ready!(self.socket.poll_write(&self.output_buffer));

            println!("Send {} bytes to {}", n, self.socket.peer_addr().unwrap());

            assert!(n > 0);

            let _ = self.output_buffer.split_to(n);
        }

        Ok(Async::Ready(()))
    }

    fn fill_read_buf(&mut self) -> Poll<(), io::Error> {
        loop {
            self.input_buffer.reserve(1024);

            let n = try_ready!(self.socket.read_buf(&mut self.input_buffer));

            if n == 0 {
                return Ok(Async::Ready(()));
            }
        }
    }
}

impl Stream for Codec {
    type Item = Payload;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let socket_closed = self.fill_read_buf()?.is_ready();

        if let None = self.input_message_size {
            if let Ok(size) = decode_length_delimiter(&self.input_buffer) {
                self.input_message_size = Some(size);
            }
        }

        match &self.input_message_size {
            Some(size) if *size >= self.input_buffer.len() => {
                let message = self.input_buffer.split_to(*size);
                self.input_message_size = None;

                if self.input_buffer.len() > 0 {
                    task::current().notify();
                }

                let payload = match GeneralMessage::decode(message) {
                    Ok(message) => message.payload,
                    Err(err) => return Err(io::Error::new(io::ErrorKind::InvalidData, err))
                };

                if let Some(payload) = payload {
                    return Ok(Async::Ready(Some(payload)));
                }
            }
            _ => ()
        };

        if socket_closed {
            Ok(Async::Ready(None))
        } else {
            Ok(Async::NotReady)
        }
    }
}
