use std::net::SocketAddr;
use std::sync::{ Mutex, Arc };

use hashbrown::HashMap;
use tokio::prelude::*;

use crate::client::{ ResponsesRx, RequestsTx };


pub struct Shared {
    pub peers: HashMap<SocketAddr, RequestsTx>
}

impl Shared {
    pub fn new() -> Shared {
        Shared {
            peers: HashMap::new()
        }
    }
}

pub struct State {
    pub shared: Arc<Mutex<Shared>>,
    pub responses_queue: ResponsesRx,
}

impl State {
    pub fn new(shared: Arc<Mutex<Shared>>, responses_queue: ResponsesRx) -> Self {
        State {
            shared,
            responses_queue,
        }
    }

    pub fn game_loop(&mut self, dt: f64) {
        const RESPONSES_PER_TICK: usize = 10;

        for _ in 0..RESPONSES_PER_TICK {
            match self.responses_queue.poll().unwrap() {
                Async::Ready(Some(data)) => {
                    //TODO: handle incoming messages properly

                    println!("Got message: {:?}", data);
                }
                _ => break
            };
        }

        println!("Loop. dt: {}s", dt);
    }
}
