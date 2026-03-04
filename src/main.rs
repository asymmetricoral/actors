enum Message {
    Add(i32),
    Subtract(i32),
    Print
}

struct Actor {
    state: i32
}

impl Actor {
    fn process(&mut self, msg: Message) {
        match msg {
            Message::Add(i) => self.state += i,
            Message::Subtract(i) => self.state -= i,
            Message::Print => println!("{}", self.state)
        }
    }
}

fn main() {
    let mut my_actor = Actor { state: 0 };

    println!("Starting actor test...");

    my_actor.process(Message::Add(10));
    my_actor.process(Message::Subtract(3));

    my_actor.process(Message::Print);
}
