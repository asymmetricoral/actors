use std::collections::VecDeque;

enum Message {
    Add(i32),
    Subtract(i32),
    Print,
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
                Message::Add(i) => self.state += i,
                Message::Subtract(i) => self.state -= i,
                Message::Print => println!("{}", self.state),
            }
        } else {
            println!("No more messages to process");
        }
    }

    fn send(&mut self, msg: Message) {
        self.messages.push_back(msg);
    }
}

fn main() {
    let mut my_actor = Actor::new(0);

    println!("Starting actor test...");

    my_actor.send(Message::Add(3));
    my_actor.send(Message::Print);
    my_actor.process();
    my_actor.send(Message::Subtract(1));
    my_actor.process();
    my_actor.process();
    my_actor.process();
}
