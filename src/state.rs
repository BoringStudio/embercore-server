use std::sync::{Arc, Mutex};

use tokio::prelude::*;

use crate::game::Game;
use crate::peer::{ConnectionEventsRx, PeersContainer, ResponsesRx, ConnectionEvent};

pub struct State {
    pub peers_container: Arc<Mutex<PeersContainer>>,
    pub responses_queue: ResponsesRx,
    pub connection_events_queue: ConnectionEventsRx,

    pub game: Game,
}

impl State {
    pub fn new(peers_container: Arc<Mutex<PeersContainer>>, responses_queue: ResponsesRx,
               connection_events_queue: ConnectionEventsRx,
    ) -> Self {
        State {
            peers_container,
            responses_queue,
            connection_events_queue,
            game: Game::new(),
        }
    }

    pub fn on_update(&mut self, dt: f64) {
        // receive connection events
        const CONNECTIONS_PER_TICK: usize = 2;

        for _ in 0..CONNECTIONS_PER_TICK {
            match self.connection_events_queue.poll().unwrap() {
                Async::Ready(Some(event)) => self.handle_connection_event(event),
                _ => break
            }
        }

        // receive client responses
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

        // game tick
        self.game.tick(dt);
    }

    fn handle_connection_event(&mut self, event: ConnectionEvent) {
        match event {
            ConnectionEvent::Connected(id) => {
                self.game.create_player(id);
            },
            ConnectionEvent::Disconnected(id) => {
                self.game.remove_player(id);
            }
        }
    }
}
