use rand::distr::{Distribution, StandardUniform};
use rand::prelude::*;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Add(i32),
    Subtract(i32),
    Spawn(i32),
    ChildSpawned(Uuid),
    Print
}

impl Distribution<Message> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Message {
        match rng.random_range(0..5) {
            0 => Message::Add(rng.random_range(1..100)),
            1 => Message::Subtract(rng.random_range(1..100)),
            2 => Message::Print,
            _ => Message::Spawn(rng.random_range(1..100))
        }
    }
}

pub enum Command {
    SpawnNewActor(i32)
}
