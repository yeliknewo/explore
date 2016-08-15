use std::sync::{mpsc};
use specs;
use nalgebra;

pub type Channel = (
    mpsc::Sender<SendEvent>,
    mpsc::Receiver<RecvEvent>
);

pub enum RecvEvent {
    Right(bool),
    Left(bool),
    Up(bool),
    Down(bool),
    Resize(u32, u32),
    MouseMoved(u32, u32),
    MouseInput(bool, ::glutin::MouseButton),
    Exit,
}

pub enum SendEvent {
    Resize,
}

#[derive(Copy, Clone)]
enum Sign {
    Pos,
    Zero,
    Neg,
}

pub struct System {
    channel: Channel,
    move_h: Sign,
    move_v: Sign,
    move_speed_mult: (f32, f32),
    resize: Option<(u32, u32)>,
    mouse_x: u32,
    mouse_y: u32,
}

impl System {
    pub fn new(
        channel: Channel,
        move_speed_mult: (f32, f32)
    ) -> System
    {
        System {
            channel: channel,
            move_h: Sign::Zero,
            move_v: Sign::Zero,
            move_speed_mult: move_speed_mult,
            resize: None,
            mouse_x: 0,
            mouse_y: 0,
        }
    }

    fn check_input(&mut self) {
        loop {
            match self.channel.1.try_recv() {
                Ok(event) => match event {
                    RecvEvent::MouseMoved(x, y) => {
                        self.mouse_x = x;
                        self.mouse_y = y;
                    },
                    RecvEvent::MouseInput(pressed, mouse_button) => {

                    },
                    RecvEvent::Right(pressed) => {
                        if pressed {
                            self.move_h = Sign::Pos;
                        } else if let Sign::Pos = self.move_h {
                            self.move_h = Sign::Zero;
                        }
                    },
                    RecvEvent::Left(pressed) => {
                        if pressed {
                            self.move_h = Sign::Neg;
                        } else if let Sign::Neg = self.move_h {
                            self.move_h = Sign::Zero;
                        }
                    },
                    RecvEvent::Up(pressed) => {
                        if pressed {
                            self.move_v = Sign::Pos;
                        } else if let Sign::Pos = self.move_v {
                            self.move_v = Sign::Zero;
                        }
                    },
                    RecvEvent::Down(pressed) => {
                        if pressed {
                            self.move_v = Sign::Neg;
                        } else if let Sign::Neg = self.move_v {
                            self.move_v = Sign::Zero;
                        }
                    },
                    RecvEvent::Resize(width, height) => {
                        self.channel.0.send(SendEvent::Resize).unwrap();
                        self.resize = Some((width, height));
                    },
                    RecvEvent::Exit => {
                        //use to save
                    }
                },
                Err(_) => return,
            }
        }
    }
}

impl<'a> specs::System<::Delta> for System {
    fn run(&mut self, arg: specs::RunArg, time: ::Delta) {
        use specs::Join;

        self.check_input();

        let mut camera = arg.fetch(|w| {
                w.write::<::comps::Camera>()
        });

        for mut c in (&mut camera).iter() {
            match (self.move_h, self.move_v) {
                (Sign::Zero, Sign::Zero) => (),
                (h, v) => {
                    let move_h = match h {
                        Sign::Pos => 1.0,
                        Sign::Zero => 0.0,
                        Sign::Neg => -1.0,
                    };
                    let move_v = match v {
                        Sign::Pos => 1.0,
                        Sign::Zero => 0.0,
                        Sign::Neg => -1.0,
                    };
                    let (x_off, y_off) = c.get_offset();
                    c.set_offset((move_h * time * self.move_speed_mult.0 + x_off, move_v * time * self.move_speed_mult.1 + y_off));
                }
            }
            if let Some((width, height)) = self.resize.take() {
                c.set_proj(nalgebra::OrthographicMatrix3::new_with_fov(width as f32 / height as f32, 90.0, 0.01, 10.0));
            }
        }
    }
}
