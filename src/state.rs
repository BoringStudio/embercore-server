use std::net::SocketAddr;

use hashbrown::HashMap;

use crate::client::Tx;


pub struct Shared {
    pub peers: HashMap<SocketAddr, Tx>
}

impl Shared {
    pub fn new() -> Shared {
        Shared {
            peers: HashMap::new()
        }
    }
}
