use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use tokio::io;
use tokio::prelude::*;
use futures::sync::mpsc;
use bytes::Bytes;

use crate::codec::Codec;
use crate::state::Shared;

pub type Tx = mpsc::UnboundedSender<Bytes>;
pub type Rx = mpsc::UnboundedReceiver<Bytes>;


pub struct Peer {
    pub codec: Codec,
    pub state: Arc<Mutex<Shared>>,
    pub rx: Rx,
    pub addr: SocketAddr,
}

impl Peer {
    pub fn new(state: Arc<Mutex<Shared>>, codec: Codec) -> Peer {
        let addr = codec.socket.peer_addr().unwrap();

        let (tx, rx) = mpsc::unbounded();

        state.lock().unwrap().peers.insert(addr, tx);

        Peer {
            codec,
            state,
            rx,
            addr,
        }
    }
}

impl Drop for Peer {
    fn drop(&mut self) {
        self.state.lock().unwrap().peers
            .remove(&self.addr);
    }
}

impl Future for Peer {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        const MESSAGES_PER_TICK: usize = 10;

        // buffer all messages for current client
        // TODO: check buffer capacity
        for i in 0..MESSAGES_PER_TICK {
            match self.rx.poll().unwrap() {
                Async::Ready(Some(v)) => {
                    self.codec.buffer(&v);

                    if i + 1 == MESSAGES_PER_TICK {
                        task::current().notify();
                    }
                }
                _ => break,
            }
        }

        // send all buffered messages
        let _ = self.codec.poll_flush()?;

        // handle all received messages
        while let Async::Ready(data) = self.codec.poll()? {
            println!("Received message: {:?}", data);

            let message = match data {
                Some(message) => message.freeze(),
                None => return Ok(Async::Ready(()))
            };

            // TODO: send message to game loop

            // now it will just broadcast it to other clients

            for (addr, tx) in &self.state.lock().unwrap().peers {
                if *addr != self.addr {
                    tx.unbounded_send(message.clone()).unwrap();
                }
            }
        }

        Ok(Async::NotReady)
    }
}