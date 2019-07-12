use std::sync::{Arc, Mutex};

use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;

use crate::codec::Codec;
use crate::peer::{Peer, PeersContainer, RequestsTx, ResponsesTx};
use crate::protocol::auth::auth_response::*;
use crate::protocol::auth::AuthResponse;
use crate::protocol::base::general_message::Payload;

enum ConnectionState {
    WaitingAuthorization,
    RespondingAuthorization(bool),
    Connected,
}

pub struct Connection {
    peer: Peer,
    requests_queue: Option<RequestsTx>,

    peers_container: Arc<Mutex<PeersContainer>>,

    state: ConnectionState,
}

impl Connection {
    pub fn new(socket: TcpStream, responses_queue: ResponsesTx,
               peers_container: Arc<Mutex<PeersContainer>>,
    ) -> Self {
        let codec = Codec::new(socket);

        let (peer, requests_queue) = Peer::new(codec, responses_queue,
                                               peers_container.clone());

        Connection {
            peer,
            requests_queue: Some(requests_queue),
            peers_container,
            state: ConnectionState::WaitingAuthorization,
        }
    }

    fn send_response(&mut self, status: Status, result: Option<Result>) {
        let response = Payload::AuthResponse(AuthResponse {
            status: status as i32,
            result,
        });

        println!("Auth response: {:?}", response);

        self.peer.codec.buffer(response);

        self.state = ConnectionState::RespondingAuthorization(status == Status::Success);
    }
}

impl Future for Connection {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use ConnectionState::*;

        loop {
            match &self.state {
                WaitingAuthorization => {
                    let payload: Payload = match try_ready!(self.peer.codec.poll()) {
                        Some(payload) => payload,
                        None => return Ok(Async::Ready(()))
                    };

                    let _credentials = match payload {
                        Payload::AuthRequest(credentials) => credentials,
                        _ => {
                            self.send_response(Status::InvalidCredentials, None);
                            continue;
                        }
                    };

                    // TODO: check credentials

                    let requests_queue = std::mem::replace(&mut self.requests_queue, None);

                    let _ = self.peers_container.lock().unwrap()
                        .try_insert_peer(&mut self.peer, requests_queue.unwrap());

                    if !self.peer.inserted() {
                        self.send_response(Status::ServerIsFull, None);
                    } else {
                        let result = Result {
                            user_id: self.peer.id.unwrap()
                        };

                        self.send_response(Status::Success, Some(result));
                    }
                }
                RespondingAuthorization(success) => {
                    let _ = try_ready!(self.peer.codec.poll_flush());

                    if *success {
                        self.state = ConnectionState::Connected;
                    } else {
                        return Ok(Async::Ready(()));
                    }
                }
                Connected => {
                    return self.peer.poll();
                }
            }
        }
    }
}
