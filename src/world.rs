use std::collections::HashMap;
use uuid::Uuid;

use crate::actor::Actor;
use crate::message::{Command, Message};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct World {
    // world owns Actor
    actors: HashMap<Uuid, Actor>,
}

impl World {
    pub fn new(capacity: usize) -> Self {
        let populated_world: HashMap<Uuid, Actor> = (0..capacity)
            .map(|_| {
                let new_uuid = Uuid::new_v4();
                let new_actor = Actor::new(0);

                (new_uuid, new_actor)
            })
            .collect();
        World {
            actors: populated_world,
        }
    }

    // until we implement garbage collection
    // remove all nodes with empty messages
    pub fn reset_dead_actors(&mut self) {
        self.actors.retain(|_, actor| actor.has_next());
    }

    pub fn current_state(&self) -> &HashMap<Uuid, Actor> {
        &self.actors
    }

    pub fn send(&mut self, target_uuid: Uuid, message: Message) {
        // https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get_mut
        if let Some(actor) = self.actors.get_mut(&target_uuid) {
            actor.send(message);
        } else {
            println!("Dead letter: Actor {} not found.", target_uuid);
        }
    }

    pub fn tick(&mut self) {
        let mut new_actors = Vec::new();
        for (tag, actor) in self.actors.iter_mut() {
            for _ in 0..3 {
                if actor.has_next() {
                    if let Some(Command::SpawnNewActor(i)) = actor.process() {
                        new_actors.push((*tag, i));
                    }
                } else {
                    break;
                }
            }
        }
        new_actors
            .into_iter()
            .for_each(|(tag, i)| self.spawn_actor(tag, i));
    }

    fn spawn_actor(&mut self, parent_uuid: Uuid, initial_state: i32) {
        let uuid: Uuid = Uuid::new_v4();
        self.actors.insert(uuid, Actor::new(initial_state));
        self.send(parent_uuid, Message::ChildSpawned(uuid));
    }
}
