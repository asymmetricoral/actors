use std::error::Error;

use rand::prelude::*;
use uuid::Uuid;

use crate::{file_service::FileService, world::World};

mod actor;
mod file_service;
mod message;
mod world;

fn main() -> Result<(), Box<dyn Error>> {
    // FileService::to_file(&my_world, "first_snapshot");
    let mut my_world = FileService::from_file("resources/first_snapshot.pulsar")?;
    if my_world.current_state().is_empty() {
        println!("Mass extinction.");
        my_world = World::new(10);
    }

    println!("Grabbing UUIDs...");
    let keyset: Vec<Uuid> = my_world.current_state().keys().copied().collect();
    for key in &keyset {
        println!("{key}");
    }

    println!("Simulation starting...");
    let mut rng = rand::rng();
    let season_length = rng.random_range(10..=80);
    
    // Calculate roughly how many messages to send per tick to hit ~1000 total
    let messages_per_tick = 1000 / season_length; 

    for _ in 0..season_length {
        let current_keys: Vec<Uuid> = my_world.current_state().keys().copied().collect();
        
        if current_keys.is_empty() {
            println!("Mass extinction hit early!");
            break; // Stop the season if everyone is dead
        }

        for _ in 0..messages_per_tick {
            let random_target = current_keys.choose(&mut rng).unwrap();
            my_world.send(*random_target, rand::random());
        }

        my_world.tick();
    }

    my_world.reset_dead_actors();
    println!("Survivors: {}", my_world.current_state().len());
    FileService::to_file(&my_world, "first_snapshot")?;
    Ok(())
}
