use gfx;
use gfx_device_gl;
use specs;
use nalgebra;
use time;
use std;

use {ReceiverHub, CompRenderType, CompTransform, CompCamera, EncoderChannel, RenderSystem, ColorFormat, DepthFormat};


pub struct Game {
    planner: specs::Planner<::Delta>,
    last_time: u64,
    player: specs::Entity,
    target_fps_delta: ::Delta,
    current_fps_delta: ::Delta,
}

impl Game {
    pub fn new(
        factory: &mut gfx_device_gl::Factory,
        (control_recv, _): ReceiverHub,
        control_send: std::sync::mpsc::Sender<()>,
        encoder_channel: EncoderChannel,
        graphics_data: (
            gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>,
            gfx::handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat>
        )
    ) -> Game
    {
        let mut planner = {
            let mut w = specs::World::new();

            w.register::<CompRenderType>();
            w.register::<CompTransform>();
            w.register::<CompCamera>();

            specs::Planner::<::Delta>::new(w, 4)
        };

        let mut renderer = RenderSystem::new(encoder_channel, graphics_data);

        let tile_render = ::art::make_square_render(&mut renderer, factory);
        let player = planner.mut_world().create_now()
            .with(CompCamera::new(
                nalgebra::Point3::new(0.0, 0.0, 2.0),
                nalgebra::Point3::new(0.0, 0.0, 0.0),
                nalgebra::Vector3::new(0.0, 1.0, 0.0),
                nalgebra::OrthographicMatrix3::new_with_fov(4.0 / 3.0, 90.0, 0.01, 10.0)
            ))
            .build();

        for y in -10..10 {
            for x in -10..10 {
                planner.mut_world().create_now()
                    .with(tile_render)
                    .with(CompTransform::new(
                        nalgebra::Isometry3::new(
                            nalgebra::Vector3::new(x as f32, y as f32, 0.0),
                            nalgebra::Vector3::new(0.0, 0.0, 0.0),
                        ),
                        nalgebra::Vector3::new(1.0, 1.0, 1.0)
                    ))
                    .build();
            }
        }

        planner.add_system(renderer, "renderer", 10);
        planner.add_system(::control::System::new(control_recv, control_send, (10.0, 10.0)), "control", 30);

        Game {
            planner: planner,
            last_time: time::precise_time_ns(),
            player: player,
            target_fps_delta: 1.0 / 60.0,
            current_fps_delta: 0.0,
        }
    }

    pub fn frame(&mut self) -> bool {
        let new_time = time::precise_time_ns();
        let delta = (new_time - self.last_time) as ::Delta / 1e9;
        self.current_fps_delta += delta;
        self.last_time = new_time;
        if self.current_fps_delta > self.target_fps_delta {
            self.planner.dispatch(self.current_fps_delta);
            self.current_fps_delta = 0.0;
        }
        std::thread::sleep(std::time::Duration::new(0, ((self.target_fps_delta - self.current_fps_delta* 0.99) * 1e9) as u32));
        // self.planner.wait();
        self.planner.mut_world().is_alive(self.player)
    }
}
