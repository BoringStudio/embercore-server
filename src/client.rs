use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use tokio::io;
use tokio::prelude::*;
use futures::sync::mpsc;

use crate::codec::Codec;
use crate::state::Shared;
use crate::protocol::base::general_message::Payload;

// Data flow: to client from server
pub type RequestsTx = mpsc::UnboundedSender<Payload>;
pub type RequestsRx = mpsc::UnboundedReceiver<Payload>;

// Data flow: to server from client
pub type ResponsesTx = mpsc::UnboundedSender<(SocketAddr, Payload)>;
pub type ResponsesRx = mpsc::UnboundedReceiver<(SocketAddr, Payload)>;


pub struct Peer {
    pub addr: SocketAddr,
    pub codec: Codec,
    pub state: Arc<Mutex<Shared>>,
    pub requests_queue: RequestsRx,
    pub responses_queue: ResponsesTx,
}

impl Peer {
    pub fn new(state: Arc<Mutex<Shared>>, codec: Codec, responses_queue: ResponsesTx) -> Peer {
        let addr = codec.socket.peer_addr().unwrap();

        let (tx, rx) = mpsc::unbounded();

        state.lock().unwrap().peers.insert(addr, tx);

        Peer {
            addr,
            codec,
            state,
            requests_queue: rx,
            responses_queue
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
        const REQUESTS_PER_TICK: usize = 10;

        // buffer all messages for current client
        for i in 0..REQUESTS_PER_TICK {
            match self.requests_queue.poll().unwrap() {
                Async::Ready(Some(data)) => {
                    self.codec.buffer(data);

                    if i + 1 == REQUESTS_PER_TICK {
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
                Some(message) => message,
                None => return Ok(Async::Ready(()))
            };

            self.responses_queue.unbounded_send((self.addr.clone(), message)).unwrap();
        }

        Ok(Async::NotReady)
    }
}