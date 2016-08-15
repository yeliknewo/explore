use gfx_device_gl;
use specs;
use nalgebra;
use time;
use std;
use std::sync::mpsc;

use {GameEventHub};

pub type Channel = (
    mpsc::Receiver<RecvEvent>
);

pub enum RecvEvent {
    Exit,
}

pub struct Game {
    planner: specs::Planner<::Delta>,
    last_time: u64,
    target_fps_delta: ::Delta,
    current_fps_delta: ::Delta,
    channel: Channel,
}

impl Game {
    pub fn new(
        factory: &mut gfx_device_gl::Factory,
        mut game_event_hub: GameEventHub
    ) -> Game
    {
        let mut planner = {
            let mut w = specs::World::new();

            w.register::<::comps::RenderType>();
            w.register::<::comps::Transform>();
            w.register::<::comps::Camera>();
            w.register::<::comps::RenderData>();

            specs::Planner::<::Delta>::new(w, 4)
        };

        let mut renderer = ::sys::render::System::new(game_event_hub.render_channel.take().unwrap());


        planner.mut_world().create_now()
            .with(::comps::Camera::new(
                nalgebra::Point3::new(0.0, 0.0, 2.0),
                nalgebra::Point3::new(0.0, 0.0, 0.0),
                nalgebra::Vector3::new(0.0, 1.0, 0.0),
                nalgebra::OrthographicMatrix3::new_with_fov(4.0 / 3.0, 90.0, 0.01, 10.0)
            ))
            .build();

        let square_render = ::art::make_square_render(&mut renderer, factory);

        for y in -10..10i32 {
            for x in -10..10i32 {
                planner.mut_world().create_now()
                    .with(square_render)
                    .with(::comps::Transform::new(
                        nalgebra::Isometry3::new(
                            nalgebra::Vector3::new(x as f32, y as f32, 0.0),
                            nalgebra::Vector3::new(0.0, 0.0, 0.0),
                        ),
                        nalgebra::Vector3::new(1.0, 1.0, 1.0)
                    ))
                    .with(::comps::RenderData::new_texture([(x + 10) as f32 / 20.0, (y + 10) as f32 / 20.0, ((x + 10) as f32 / 20.0 + (y + 10) as f32 / 20.0) / 2.0, 1.0]))
                    .build();
            }
        }

        planner.add_system(renderer, "renderer", 10);
        planner.add_system(::sys::control::System::new(game_event_hub.control_channel.take().unwrap(), (10.0, 10.0)), "control", 30);

        Game {
            planner: planner,
            last_time: time::precise_time_ns(),
            target_fps_delta: 1.0 / 60.0,
            current_fps_delta: 0.0,
            channel: game_event_hub.game_channel.take().unwrap(),
        }
    }

    pub fn frame(&mut self) -> bool {
        let new_time = time::precise_time_ns();
        let delta = (new_time - self.last_time) as ::Delta / 1e9;
        self.current_fps_delta += delta;
        self.last_time = new_time;

        match self.channel.try_recv() {
            Err(mpsc::TryRecvError::Empty) => {
                if self.current_fps_delta > self.target_fps_delta {
                    self.planner.dispatch(self.current_fps_delta);
                    println!("Estimated FPS: {}", self.current_fps_delta * 60.0 * 60.0);
                    self.current_fps_delta = 0.0;
                } else {
                    std::thread::sleep(std::time::Duration::new(0, ((self.target_fps_delta - self.current_fps_delta* 0.99) * 1e9) as u32));
                }
                true
            },
            Ok(RecvEvent::Exit) |
            Err(mpsc::TryRecvError::Disconnected) => {
                self.planner.wait();
                false
            },
        }
    }
}
