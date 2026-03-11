use std::collections::VecDeque;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::message::{Command, Message};

#[derive(Serialize, Deserialize, Debug)]
pub struct Actor {
    state: i32,
    messages: VecDeque<Message>,
    children: Vec<Uuid>,
}

impl Actor {
    pub fn new(initial_state: i32) -> Self {
        Self {
            state: initial_state,
            messages: VecDeque::new(),
            children: Vec::new(),
        }
    }

    pub fn process(&mut self) -> Option<Command> {
        if let Some(msg) = self.messages.pop_front() {
            match msg {
                Message::Add(i) => self.state = self.state.saturating_add(i),
                Message::Subtract(i) => self.state = self.state.saturating_sub(i),
                Message::Print => print!("{} ", self.state),
                Message::Spawn(i) => return Some(Command::SpawnNewActor(i)),
                Message::ChildSpawned(uuid) => self.children.push(uuid),
            }
        } else {
            println!("No more messages to process");
        }
        None
    }

    pub fn send(&mut self, msg: Message) {
        self.messages.push_back(msg);
    }

    pub fn has_next(&self) -> bool {
        !self.messages.is_empty()
    }
}
