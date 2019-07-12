use specs::{Read, ReadStorage, System, WriteStorage};
use specs::Join;

use crate::game::components::{Position, Velocity};
use crate::game::resources::ServerClock;

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        Read<'a, ServerClock>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Velocity>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            server_clock,
            mut positions,
            velocities
        ) = data;

        for (pos, vel) in Join::join((&mut positions, &velocities)) {
            pos.x += vel.x * server_clock.0;
            pos.y += vel.y * server_clock.0;

            println!("MovementSystem: moved smth to {:?}", pos);
        }
    }
}
