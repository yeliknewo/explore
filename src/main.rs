extern crate gfx_core;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate nalgebra;
extern crate specs;

mod graphics;
mod transform;
mod camera;

use std::sync::{mpsc};

use graphics::{Graphics, RenderSystem, CompRenderType, EncoderChannel};
use transform::{CompTransform};
use camera::{CompCamera};

fn main() {
    let (mut graphics, mut encoder) = Graphics::<gfx_device_gl::Resources, gfx_device_gl::Factory>::new_glutin();

    let mut planner = {
        let mut w = specs::World::new();

        w.register::<CompRenderType>();
        w.register::<CompTransform>();
        w.register::<CompCamera>();

        specs::Planner::<()>::new(w, 4)
    };

    let (game_send, dev_recv) = mpsc::channel();
    let (dev_send, game_recv) = mpsc::channel();

    let encoder_channel = EncoderChannel {
        receiver: game_recv,
        sender: game_send,
    };

    let mut renderer = graphics.make_render_system::<gfx_device_gl::CommandBuffer>(encoder_channel);

    planner.add_system(renderer, "renderer", 10);
}
