use std::sync::{mpsc};
use gfx;
use gfx_device_gl;
use specs;
use nalgebra;

use {ColorFormat, DepthFormat};

pub enum Event {
    Right(bool),
    Left(bool),
    Up(bool),
    Down(bool),
    Resize(u32, u32),
}

#[derive(Copy, Clone)]
enum Sign {
    Pos,
    Zero,
    Neg,
}

pub struct System {
    control_recv: mpsc::Receiver<Event>,
    control_send: mpsc::Sender<(
        gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>,
        gfx::handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat>
    )>,
    move_h: Sign,
    move_v: Sign,
    move_speed_mult: (f32, f32),
    resize: Option<(u32, u32)>,
}

impl System {
    pub fn new(
        control_recv: mpsc::Receiver<Event>,
        control_send: mpsc::Sender<(
            gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>,
            gfx::handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat>
        )>,
        move_speed_mult: (f32, f32)
    ) -> System
    {
        System {
            control_recv: control_recv,
            control_send: control_send,
            move_h: Sign::Zero,
            move_v: Sign::Zero,
            move_speed_mult: move_speed_mult,
            resize: None,
        }
    }

    fn check_input(&mut self) {
        loop {
            match self.control_recv.try_recv() {
                Ok(event) => match event {
                    Event::Right(pressed) => {
                        if pressed {
                            self.move_h = Sign::Pos;
                        } else if let Sign::Pos = self.move_h {
                            self.move_h = Sign::Zero;
                        }
                    },
                    Event::Left(pressed) => {
                        if pressed {
                            self.move_h = Sign::Neg;
                        } else if let Sign::Neg = self.move_h {
                            self.move_h = Sign::Zero;
                        }
                    },
                    Event::Up(pressed) => {
                        if pressed {
                            self.move_v = Sign::Pos;
                        } else if let Sign::Pos = self.move_v {
                            self.move_v = Sign::Zero;
                        }
                    },
                    Event::Down(pressed) => {
                        if pressed {
                            self.move_v = Sign::Neg;
                        } else if let Sign::Neg = self.move_v {
                            self.move_v = Sign::Zero;
                        }
                    },
                    Event::Resize(width, height) => self.resize = Some((width, height)),
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
                self.control_send.send().unwrap();
            }
        }
    }
}
