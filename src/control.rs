use std::sync::{mpsc};
use specs;

pub enum Event {

}

pub struct System {
    channel: mpsc::Receiver<Event>,
}

impl System {
    pub fn new(channel: mpsc::Receiver<Event>) -> System {
        System {
            channel: channel,
        }
    }

    fn check_input(&mut self) {
        loop {
            match self.channel.try_recv() {
                Ok(event) => match event {

                },
                Err(_) => return,
            }
        }
    }
}

impl specs::System<::Delta> for System {
    fn run(&mut self, arg: specs::RunArg, time: ::Delta) {
        use specs::Join;
        self.check_input();
    }
}
