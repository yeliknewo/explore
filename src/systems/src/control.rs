pub type Channel = (
    ::std::sync::mpsc::Sender<SendEvent>,
    ::std::sync::mpsc::Receiver<RecvEvent>
);

#[derive(Debug)]
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

#[derive(Debug)]
pub enum SendEvent {
    Resize,
    Error(::utils::Error),
}

#[derive(Debug, Copy, Clone)]
enum Sign {
    Pos,
    Zero,
    Neg,
}

#[derive(Debug)]
pub struct System {
    channel: Channel,
    move_h: Sign,
    move_v: Sign,
    move_speed_mult: ::math::Point2,
    resize: Vec<(u32, u32)>,
    mouse_location: ::math::Point2,
    mouse_button: Vec<(bool, ::glutin::MouseButton)>,
    screen_resolution: ::math::Point2,
    ortho_helper: ::math::OrthographicHelper,
}

impl System {
    pub fn new(
        channel: Channel,
        move_speed_mult: ::math::Point2,
        mouse_location: ::math::Point2,
        screen_resolution: ::math::Point2,
        ortho_helper: ::math::OrthographicHelper
    ) -> System
    {
        System {
            channel: channel,
            move_h: Sign::Zero,
            move_v: Sign::Zero,
            move_speed_mult: move_speed_mult,
            resize: vec!(),
            mouse_location: mouse_location,
            mouse_button: vec!(),
            screen_resolution: screen_resolution,
            ortho_helper: ortho_helper,
        }
    }

    fn check_input(&mut self) {
        loop {
            match self.channel.1.try_recv() {
                Ok(event) => match event {
                    RecvEvent::MouseMoved(x, y) => {
                        self.mouse_location = ::math::Point2::new(
                            x as ::utils::Coord / self.screen_resolution.get_x(),
                            y as ::utils::Coord / self.screen_resolution.get_y()
                        );
                    },
                    RecvEvent::MouseInput(pressed, mouse_button) => self.mouse_button.push((pressed, mouse_button)),
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
                        match self.channel.0.send(SendEvent::Resize) {
                            Ok(()) => (),
                            Err(err) => error!("resize channel 0 send error: {}", err),
                        };
                        self.resize.push((width, height));
                    },
                    RecvEvent::Exit => {
                        //use to save
                    },
                },
                Err(_) => return,
            }
        }
    }
}

impl<'a> ::specs::System<::utils::Delta> for System {
    fn run(&mut self, arg: ::specs::RunArg, time: ::utils::Delta) {
        use specs::Join;

        self.check_input();

        let (mut camera, mut clickable, mut texture_data, transform) = arg.fetch(|w| {
            (w.write::<::comps::Camera>(), w.write::<::comps::Clickable>(), w.write::<::comps::RenderData>(), w.read::<::comps::Transform>())
        });

        let mut camera_opt = None;

        for mut c in (&mut camera).iter() {
            if c.is_main() {
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
                        let offset = c.get_offset();
                        c.set_offset(::math::Point2::new(
                            move_h * time * self.move_speed_mult.get_x() + offset.get_x(),
                            move_v * time * self.move_speed_mult.get_y() + offset.get_y()
                        ));
                    },
                }
                for &(width, height) in &self.resize {
                    self.ortho_helper.set_aspect_ratio(width as ::utils::Coord / height as ::utils::Coord);
                    c.set_proj(&self.ortho_helper);
                    self.screen_resolution = ::math::Point2::new(width as ::utils::Coord, height as ::utils::Coord);
                }
                camera_opt = Some(c);
                break;
            }
        }

        let camera = match camera_opt {
            Some(c) => c,
            None => {
                error!("run camera opt was none");
                self.channel.0.send(SendEvent::Error(::utils::Error::Logged)).unwrap();
                return;
            }
        };

        if let Some(input) = self.mouse_button.pop() {
            match input {
                (true, ::glutin::MouseButton::Left) => {
                    for (t, mut c, mut td) in (&transform, &mut clickable, &mut texture_data).iter() {
                        if  c.hitbox.check_collide_point_with_offset(camera.screen_to_world_point(self.mouse_location), t.get_gui_offset()) {
                            c.clicked = true;
                            td.set_tint([1.0, 1.0, 1.0, 1.0]);
                        } else {
                            c.clicked = false;
                        }
                    }
                },
                _ => (),
            }
        }
    }
}
