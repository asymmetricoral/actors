use rand::distr::{Distribution, StandardUniform};
use rand::prelude::*;
use std::collections::HashMap;
use std::collections::VecDeque;
use uuid::Uuid;

#[derive(Debug)]
enum Message {
    Add(i32),
    Subtract(i32),
    Print,
}

impl Distribution<Message> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Message {
        match rng.random_range(0..5) {
            0 => Message::Add(rng.random_range(1..100)),
            1 => Message::Subtract(rng.random_range(1..100)),
            _ => Message::Print,
        }
    }
}

struct Actor {
    state: i32,
    messages: VecDeque<Message>,
}

impl Actor {
    fn new(initial_state: i32) -> Self {
        Self {
            state: initial_state,
            messages: VecDeque::new(),
        }
    }

    fn process(&mut self) {
        if let Some(msg) = self.messages.pop_front() {
            match msg {
                Message::Add(i) => self.state = self.state.saturating_add(i),
                Message::Subtract(i) => self.state = self.state.saturating_sub(i),
                Message::Print => println!("{}", self.state),
            }
        } else {
            println!("No more messages to process");
        }
    }

    fn send(&mut self, msg: Message) {
        self.messages.push_back(msg);
    }

    fn has_next(&self) -> bool {
        !self.messages.is_empty()
    }
}

struct World {
    // world owns Actor
    actors: HashMap<Uuid, Actor>,
}

impl World {
    fn new(capacity: usize) -> Self {
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

    fn current_state(&self) -> &HashMap<Uuid, Actor> {
        &self.actors
    }

    fn send(&mut self, target_uuid: Uuid, message: Message) {
        // https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get_mut
        if let Some(actor) = self.actors.get_mut(&target_uuid) {
            actor.send(message);
        } else {
            println!("Dead letter: Actor {} not found.", target_uuid);
        }
    }

    fn tick(&mut self) {
        for actor in self.actors.values_mut() {
            for _ in 0..3 {
                if actor.has_next() {
                    actor.process();
                } else {
                    break;
                }
            }
        }
    }
}

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
