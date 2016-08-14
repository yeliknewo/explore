#[macro_use]
extern crate gfx;
// extern crate gfx_core;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate nalgebra;
extern crate specs;
extern crate time;

mod art;
mod game;
mod graphics;
mod event;

mod control;

mod transform;
mod camera;

use std::sync::{mpsc};
use gfx::{Typed};
use gfx::format::{Formatted};

pub use graphics::{Vertex, Index, RenderSystem, CompRenderType, EncoderChannel, ColorFormat, DepthFormat};
pub use event::{ReceiverHub};
pub use transform::{CompTransform};
pub use camera::{CompCamera};

pub type Delta = f32;

fn main() {
    let (graphics_data, mut factory, encoder, window, mut device) = graphics::build_graphics();

    let mut dimensions = graphics_data.0.get_dimensions();

    let (event_send, event_recv) = event::SenderHub::new();
    let (game_send, dev_recv) = mpsc::channel();
    let (dev_send, game_recv) = mpsc::channel();
    let (control_send, control_recv) = mpsc::channel();

    game_send.send(encoder.clone_empty()).unwrap();
    game_send.send(encoder).unwrap();

    let encoder_channel = EncoderChannel {
        receiver: game_recv,
        sender: game_send,
    };

    let game = game::Game::new(&mut factory, event_recv, control_send, encoder_channel, graphics_data);

    std::thread::spawn(|| {
        let mut game = game;
        while game.frame() {}
    });


    'main: loop {
        use gfx::Device;
        let mut encoder = match dev_recv.recv() {
            Ok(r) => r,
            Err(_) => break 'main,
        };

        for event in window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => break 'main,
                _ => event_send.process_glutin(event),
            }
        }

        if let Ok(()) = control_recv.try_recv() {
            if let Some((color_view, depth_view)) = gfx_window_glutin::update_views_raw(&window, dimensions, ColorFormat::get_format(), DepthFormat::get_format()) {
                let mut render_target_view = gfx::handle::RenderTargetView::<gfx_device_gl::Resources, ColorFormat>::new(color_view);
                dimensions = render_target_view.get_dimensions();
                gfx_window_glutin::update_views(
                    &window,
                    &mut render_target_view,
                    &mut gfx::handle::DepthStencilView::<gfx_device_gl::Resources, DepthFormat>::new(depth_view)
                );
            }
        }

        encoder.flush(&mut device);
        dev_send.send(encoder).unwrap();
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
