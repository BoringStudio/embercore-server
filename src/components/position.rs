use glm::Vec2;
use serde::export::Formatter;
use specs::prelude::*;

use std::fmt::{self, Debug};

#[derive(Default, Component, Clone)]
#[storage(VecStorage)]
pub struct Position(pub Vec2);

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }
}

impl From<Vec2> for Position {
    fn from(v: Vec2) -> Self {
        Self(v)
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
