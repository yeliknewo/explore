use gfx;
use gfx_device_gl;
use specs;
use time;

use {ReceiverHub, CompRenderType, CompTransform, CompCamera, EncoderChannel, RenderSystem, ColorFormat, DepthFormat};


pub struct Game {
    planner: specs::Planner<::Delta>,
    last_time: u64,
    player: specs::Entity,
}

impl Game {
    pub fn new(
        factory: &mut gfx_device_gl::Factory,
        (control_recv, _): ReceiverHub,
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

        let player_render = ::art::make_player_render(&mut renderer, factory);
        let player = planner.mut_world().create_now()
            .with(player_render)
            .build();

        planner.add_system(renderer, "renderer", 10);
        planner.add_system(::control::System::new(control_recv), "control", 30);

        Game {
            planner: planner,
            last_time: time::precise_time_ns(),
            player: player,
        }
    }

    pub fn frame(&mut self) -> bool {
        let new_time = time::precise_time_ns();
        let delta = (new_time - self.last_time) as ::Delta / 1e9;
        self.last_time = new_time;
        self.planner.dispatch(delta);
        self.planner.mut_world().is_alive(self.player)
    }
}
