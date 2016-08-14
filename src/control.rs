use std::sync::{mpsc};
use specs;

pub enum Event {
    Right(bool),
    Left(bool),
    Up(bool),
    Down(bool),
}

pub struct System {
    channel: mpsc::Receiver<Event>,
    move_h: f32,
    move_v: f32,
    move_speed_mult: (f32, f32),
}

impl System {
    pub fn new(channel: mpsc::Receiver<Event>, move_speed_mult: (f32, f32)) -> System {
        System {
            channel: channel,
            move_h: 0.0,
            move_v: 0.0,
            move_speed_mult: move_speed_mult,
        }
    }

    fn check_input(&mut self) {
        loop {
            match self.channel.try_recv() {
                Ok(event) => match event {
                    Event::Right(pressed) => {
                        if pressed {
                            self.move_h = 1.0;
                        } else if self.move_h == 1.0 {
                            self.move_h = 0.0;
                        }
                    },
                    Event::Left(pressed) => {
                        if pressed {
                            self.move_h = -1.0;
                        } else if self.move_h == -1.0 {
                            self.move_h = 0.0;
                        }
                    },
                    Event::Up(pressed) => {
                        if pressed {
                            self.move_v = 1.0;
                        } else if self.move_v == 1.0 {
                            self.move_v = 0.0;
                        }
                    },
                    Event::Down(pressed) => {
                        if pressed {
                            self.move_v = -1.0;
                        } else if self.move_v == -1.0 {
                            self.move_v = 0.0;
                        }
                    },
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
        let mut camera = arg.fetch(|w| {
                w.write::<::camera::CompCamera>()
        });

        for mut c in (&mut camera).iter() {
            let (x_off, y_off) = c.get_offset();
            c.set_offset((self.move_h * time * self.move_speed_mult.0 + x_off, self.move_v * time * self.move_speed_mult.1 + y_off));
        }
    }
}
