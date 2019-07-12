use std::sync::{Arc, Mutex};
use std::u32;

use futures::sync::mpsc;
use tokio::io;
use tokio::prelude::*;

use crate::codec::Codec;
use crate::peer::PeersContainer;
use crate::protocol::base::general_message::Payload;

// Data flow: to client from server
pub type RequestsTx = mpsc::UnboundedSender<Payload>;
pub type RequestsRx = mpsc::UnboundedReceiver<Payload>;

// Data flow: to server from client
pub type ResponsesTx = mpsc::UnboundedSender<(u32, Payload)>;
pub type ResponsesRx = mpsc::UnboundedReceiver<(u32, Payload)>;


pub struct Peer {
    pub id: Option<u32>,
    pub codec: Codec,
    requests_queue: RequestsRx,
    responses_queue: ResponsesTx,

    peers_container: Arc<Mutex<PeersContainer>>,
}

impl Peer {
    pub fn new(codec: Codec, responses_queue: ResponsesTx,
               peers_container: Arc<Mutex<PeersContainer>>,
    ) -> (Peer, RequestsTx) {
        let (tx, rx) = mpsc::unbounded();

        (Peer {
            id: None,
            codec,
            requests_queue: rx,
            responses_queue,
            peers_container,
        }, tx)
    }

    pub fn inserted(&self) -> bool {
        self.id != None
    }
}

impl Drop for Peer {
    fn drop(&mut self) {
        let id = match self.id {
            Some(id) => id,
            None => return
        };

        self.peers_container.lock().unwrap().try_remove_peer(id);
    }
}

impl Future for Peer {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        if !self.inserted() {
            return Ok(Async::Ready(()));
        }

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
            let message = match data {
                Some(message) => message,
                None => return Ok(Async::Ready(()))
            };

            self.responses_queue.unbounded_send((self.id.unwrap(), message)).unwrap();
        }

        Ok(Async::NotReady)
    }
}
