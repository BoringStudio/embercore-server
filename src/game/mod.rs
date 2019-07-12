use specs::{Builder, RunNow, World, WorldExt};

use components::*;
use resources::*;
use systems::*;

pub mod components;
pub mod resources;
pub mod systems;

pub struct Game {
    world: World,
    movement_system: MovementSystem,
}

impl Game {
    pub fn new() -> Self {
        let mut world = World::new();

        world.insert(ServerClock(0.0));

        world.register::<Player>();
        world.register::<Position>();
        world.register::<Velocity>();

        Game {
            world,
            movement_system: MovementSystem,
        }
    }

    pub fn tick(&mut self, dt: f64) {
        self.world.insert(ServerClock(dt));

        self.world.maintain();

        self.movement_system.run_now(&mut self.world);
    }

    pub fn create_player(&mut self, id: u32) {
        self.world.create_entity()
            .with::<Player>(Player { id })
            .with::<Position>(Position { x: 0.0, y: 0.0 })
            .with::<Velocity>(Velocity { x: 0.0, y: 0.0 })
            .build();

        println!("Player connected: {}", id);
    }

    pub fn remove_player(&mut self, id: u32) {
        let player = {
            let entities = self.world.entities();

            let players = self.world.read_component::<Player>();

            specs::Join::join((&*entities, &players))
                .find(|&(_, player)| player.id == id)
                .map(|(entity, _)| entity)
        };

        if let Some(player) = player {
            self.world.delete_entity(player).unwrap();
        }

        println!("Player disconnected: {}", id);
    }
}
