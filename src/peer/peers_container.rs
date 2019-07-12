use futures::sync::mpsc;
use rust_dense_bitset::{BitSet, DenseBitSet};

use crate::peer::{Peer, RequestsTx};

pub type ConnectionEventsTx = mpsc::UnboundedSender<ConnectionEvent>;
pub type ConnectionEventsRx = mpsc::UnboundedReceiver<ConnectionEvent>;

pub enum ConnectionEvent {
    Connected(u32),
    Disconnected(u32),
}

pub struct PeersContainer {
    peers: Vec<Option<RequestsTx>>,
    slots: DenseBitSet,

    connection_events_queue: ConnectionEventsTx,
}

impl PeersContainer {
    pub fn new(connection_events_queue: ConnectionEventsTx) -> PeersContainer {
        let slots = !DenseBitSet::new();

        PeersContainer {
            peers: vec![None; 64],
            slots,
            connection_events_queue,
        }
    }

    pub fn try_insert_peer(&mut self, peer: &mut Peer, requests_queue: RequestsTx) -> bool {
        let first_empty = self.slots.first_set();

        if first_empty as usize == self.peers.len() || peer.inserted() {
            return false;
        }

        peer.id = Some(first_empty);

        self.connection_events_queue
            .unbounded_send(ConnectionEvent::Connected(first_empty))
            .unwrap();

        self.peers[first_empty as usize] = Some(requests_queue);
        self.slots.set_bit(first_empty as usize, false);

        true
    }

    pub fn try_remove_peer(&mut self, id: u32) -> bool {
        if !self.has_peer(id) {
            return false;
        }

        self.connection_events_queue
            .unbounded_send(ConnectionEvent::Disconnected(id))
            .unwrap();

        self.peers[id as usize] = None;
        self.slots.set_bit(id as usize, true);

        true
    }

    pub fn has_peer(&self, id: u32) -> bool {
        !self.slots.get_bit(id as usize)
    }

    pub fn empty(&self) -> bool {
        self.slots.all()
    }

    pub fn full(&self) -> bool {
        self.slots.none()
    }
}

impl<'c> IntoIterator for &'c PeersContainer {
    type Item = (u32, &'c RequestsTx);
    type IntoIter = PeersContainerIntoIterator<'c>;

    fn into_iter(self) -> Self::IntoIter {
        PeersContainerIntoIterator {
            container: self,
            index: 0,
        }
    }
}

pub struct PeersContainerIntoIterator<'c> {
    container: &'c PeersContainer,
    index: u32,
}

impl<'c> Iterator for PeersContainerIntoIterator<'c> {
    type Item = (u32, &'c RequestsTx);

    fn next(&mut self) -> Option<Self::Item> {
        let peers = &self.container.peers;

        while (self.index as usize) < peers.len() && !self.container.has_peer(self.index) {
            self.index += 1;
        }

        if (self.index as usize) >= peers.len() {
            return None;
        }

        let index = self.index;
        self.index += 1;

        return Some((index, self.container.peers[index as usize].as_ref().unwrap()));
    }
}
