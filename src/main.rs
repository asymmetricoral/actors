use rand::prelude::*;
use uuid::Uuid;

use crate::world::World;

mod actor;
mod message;
mod world;

fn main() {
    let mut my_world = World::new(10);

    println!("Grabbing UUIDs...");
    let keyset: Vec<Uuid> = my_world.current_state().keys().copied().collect();
    for key in &keyset {
        println!("{key}");
    }

    println!("Sending random messages to actors...");
    let mut rng = rand::rng();
    for _ in 0..1000 {
        let random_target = keyset.choose(&mut rng).unwrap();
        my_world.send(*random_target, rand::random());
    }

    for _ in 0..1000 {
        my_world.tick();
    }
}
