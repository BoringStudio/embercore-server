use std::sync::{Arc, Mutex};

use tokio::prelude::*;

use crate::game::Game;
use crate::peer::{PeersContainer, ResponsesRx};

pub struct State {
    pub peers_container: Arc<Mutex<PeersContainer>>,
    pub responses_queue: ResponsesRx,

    pub game: Game,
}

impl State {
    pub fn new(peers_container: Arc<Mutex<PeersContainer>>, responses_queue: ResponsesRx) -> Self {
        State {
            peers_container,
            responses_queue,
            game: Game::new(),
        }
    }

    pub fn on_init(&mut self) {}

    pub fn on_update(&mut self, dt: f64) {
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

        self.game.tick(dt);
    }
}
