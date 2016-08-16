pub type Channel = (
    ::std::sync::mpsc::Sender<SendEvent>,
    ::std::sync::mpsc::Receiver<RecvEvent>
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
    move_speed_mult: (::utils::Coord, ::utils::Coord),
    resize: Option<(u32, u32)>,
    screen_size: ::math::Point2,
    mouse_location: ::math::Point2,
    mouse_button: Vec<(bool, ::glutin::MouseButton)>,
    view_size: f32,
}

impl System {
    pub fn new(
        channel: Channel,
        move_speed_mult: (::utils::Coord, ::utils::Coord),
        screen_size: ::math::Point2
    ) -> System
    {
        System {
            channel: channel,
            move_h: Sign::Zero,
            move_v: Sign::Zero,
            move_speed_mult: move_speed_mult,
            resize: None,
            screen_size: screen_size,
            mouse_location: ::math::Point2::new(0.0, 0.0),
            mouse_button: vec!(),
            view_size: 10.0,
        }
    }

    fn check_input(&mut self) {
        loop {
            match self.channel.1.try_recv() {
                Ok(event) => match event {
                    RecvEvent::MouseMoved(x, y) => {
                        self.mouse_location = ::math::Point2::new(
                            x as f32 / self.screen_size.get_x() * self.view_size,
                            y as f32 / self.screen_size.get_y() * self.view_size
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

impl<'a> ::specs::System<::utils::Delta> for System {
    fn run(&mut self, arg: ::specs::RunArg, time: ::utils::Delta) {
        use specs::Join;

        self.check_input();

        let (mut camera, mut clickable, mut texture_data, transform) = arg.fetch(|w| {
            (w.write::<::comps::Camera>(), w.write::<::comps::Clickable>(), w.write::<::comps::RenderData>(), w.read::<::comps::Transform>())
        });

        if let Some(input) = self.mouse_button.pop() {
            trace!("Mouse Button Received");
            match input {
                (true, ::glutin::MouseButton::Left) => {
                    trace!("Left Mouse Button Pressed");
                    for (t, mut c, mut td) in (&transform, &mut clickable, &mut texture_data).iter() {
                        trace!("Found Entity with Clickable and Texture Data");
                        if  c.hitbox.check_collide_point_with_offset(self.mouse_location, t.get_gui_offset()) {
                            trace!("Hitbox Collided with Mouse Location");
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
                },
            }
            if let Some((width, height)) = self.resize.take() {
                c.set_proj(::nalgebra::OrthographicMatrix3::new_with_fov(width as ::utils::Coord / height as ::utils::Coord, 90.0, 0.0, 10.0));
            }
        }
    }
}
