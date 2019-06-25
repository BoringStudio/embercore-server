use tokio::io;
use tokio::prelude::*;
use tokio::net::TcpStream;
use bytes::{BytesMut, BufMut};


pub struct Codec {
    pub socket: TcpStream,
    pub rd: BytesMut,
    pub wr: BytesMut,
}

impl Codec {
    pub fn new(socket: TcpStream) -> Self {
        Codec {
            socket,
            rd: BytesMut::new(),
            wr: BytesMut::new(),
        }
    }

    pub fn buffer(&mut self, line: &[u8]) {
        self.wr.reserve(line.len());

        self.wr.put(line);
    }

    pub fn poll_flush(&mut self) -> Poll<(), io::Error> {
        while !self.wr.is_empty() {
            let n = try_ready!(self.socket.poll_write(&self.wr));

            println!("Send {} bytes to {}", n, self.socket.peer_addr().unwrap());

            assert!(n > 0);

            let _ = self.wr.split_to(n);
        }

        Ok(Async::Ready(()))
    }

    fn fill_read_buf(&mut self) -> Poll<(), io::Error> {
        loop {
            self.rd.reserve(1024);

            let n = try_ready!(self.socket.read_buf(&mut self.rd));

            if n == 0 {
                return Ok(Async::Ready(()));
            }
        }
    }
}

impl Stream for Codec {
    type Item = BytesMut;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let socket_closed = self.fill_read_buf()?.is_ready();

        let pos = self.rd
            .windows(2)
            .position(|bytes| bytes == b"\r\n");

        if let Some(pos) = pos {
            let mut line = self.rd.split_to(pos + 2);
            line.split_off(pos);

            return Ok(Async::Ready(Some(line)));
        }

        if socket_closed {
            Ok(Async::Ready(None))
        } else {
            Ok(Async::NotReady)
        }
    }
}
