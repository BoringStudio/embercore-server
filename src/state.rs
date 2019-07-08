use std::net::SocketAddr;
use std::sync::{ Mutex, Arc };

use hashbrown::HashMap;
use tokio::prelude::*;

use crate::client::{ ResponsesRx, RequestsTx };
use rlua::{Lua, Table, Function};
use rlua::prelude::LuaTable;


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

    pub lua: Lua,
}

impl State {
    pub fn new(shared: Arc<Mutex<Shared>>, responses_queue: ResponsesRx) -> Self {
        let lua = Lua::new();

        State {
            shared,
            responses_queue,
            lua,
        }
    }

    pub fn on_init(&mut self) {
        let main_script = std::fs::read_to_string("./res/main.lua").unwrap();

        self.lua.context(|context| {
            let entry_scene: Table = context
                .load(&main_script)
                .eval::<Table>().unwrap();

            entry_scene.get::<_, Function>("on_init").unwrap().call::<_, ()>(()).unwrap();
        });

        println!("Script: {}", main_script);
    }

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
    }
}
