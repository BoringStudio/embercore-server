use specs::{Builder, RunNow, World, WorldExt};

use movement::*;
use player::*;

pub mod movement;
pub mod player;

pub struct Game {
    world: World,
    movement_system: MovementSystem,
}

impl Game {
    pub fn new() -> Self {
        let mut world = World::new();

        world.register::<Position>();
        world.register::<Velocity>();

        Game {
            world,
            movement_system: MovementSystem
        }
    }

    pub fn tick(&mut self, dt: f64) {
        self.movement_system.run_now(&mut self.world);

        // println!("Dt: {}", dt);
    }

    pub fn create_player(&mut self, id: u32) {
        let player = Player {
            id
        };

        let position = Position {
            x: 0.0,
            y: 0.0
        };

        let velocity = Velocity {
            x: 0.0,
            y: 0.0
        };

        self.world.create_entity()
            .with::<Player>(player)
            .with::<Position>(position)
            .with::<Velocity>(velocity)
            .build();

        self.world.maintain();
    }
}
