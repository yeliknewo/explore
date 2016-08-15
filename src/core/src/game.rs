pub type Channel = (
    ::std::sync::mpsc::Receiver<RecvEvent>
);

pub enum RecvEvent {
    Exit,
}

pub struct Game {
    planner: ::specs::Planner<::utils::Delta>,
    last_time: u64,
    target_fps_delta: ::utils::Delta,
    current_fps_delta: ::utils::Delta,
    channel: Channel,
}

impl Game {
    pub fn new<F>(
        factory: &mut ::gfx_device_gl::Factory,
        mut game_event_hub: ::event::GameEventHub,
        screen_size: ::math::Point2,
        setup: F,
    ) -> Game
    where F: for<'a> Fn(&'a mut ::specs::Planner<::utils::Delta>, &'a mut ::sys::render::System, &'a mut ::gfx_device_gl::Factory)
    {
        let mut planner = {
            let mut w = ::specs::World::new();

            w.register::<::comps::RenderType>();
            w.register::<::comps::Transform>();
            w.register::<::comps::Camera>();
            w.register::<::comps::RenderData>();
            w.register::<::comps::Clickable>();

            ::specs::Planner::<::utils::Delta>::new(w, 4)
        };

        let mut renderer = ::sys::render::System::new(game_event_hub.render_channel.take().unwrap());

        setup(&mut planner, &mut renderer, factory);

        planner.add_system(renderer, "renderer", 10);
        planner.add_system(::sys::control::System::new(game_event_hub.control_channel.take().unwrap(), (10.0, 10.0), screen_size), "control", 30);

        Game {
            planner: planner,
            last_time: ::time::precise_time_ns(),
            target_fps_delta: 1.0 / 60.0,
            current_fps_delta: 0.0,
            channel: game_event_hub.game_channel.take().unwrap(),
        }
    }

    pub fn frame(&mut self) -> bool {
        let new_time = ::time::precise_time_ns();
        let delta = (new_time - self.last_time) as ::utils::Delta / 1e9;
        self.current_fps_delta += delta;
        self.last_time = new_time;

        match self.channel.try_recv() {
            Err(::std::sync::mpsc::TryRecvError::Empty) => {
                if self.current_fps_delta > self.target_fps_delta {
                    self.planner.dispatch(self.current_fps_delta);
                    // println!("Estimated FPS: {}", self.current_fps_delta * 60.0 * 60.0);
                    self.current_fps_delta = 0.0;
                } else {
                    ::std::thread::sleep(::std::time::Duration::new(0, ((self.target_fps_delta - self.current_fps_delta* 0.99) * 1e9) as u32));
                }
                true
            },
            Ok(RecvEvent::Exit) |
            Err(::std::sync::mpsc::TryRecvError::Disconnected) => {
                self.planner.wait();
                false
            },
        }
    }
}
