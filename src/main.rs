#[macro_use]
extern crate gfx;
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

pub use graphics::{RenderSystem, CompRenderType, EncoderChannel, ColorFormat, DepthFormat};
pub use event::{ReceiverHub};
pub use transform::{CompTransform};
pub use camera::{CompCamera};

pub type Delta = f32;

fn main() {
    let (graphics_data, mut factory, encoder, window, mut device) = graphics::build_graphics();


    let (event_send, event_recv) = event::SenderHub::new();
    let (game_send, dev_recv) = mpsc::channel();
    let (dev_send, game_recv) = mpsc::channel();

    game_send.send(encoder.clone_empty()).unwrap();
    game_send.send(encoder).unwrap();

    let encoder_channel = EncoderChannel {
        receiver: game_recv,
        sender: game_send,
    };

    let game = game::Game::new(&mut factory, event_recv, encoder_channel, graphics_data);

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

        encoder.flush(&mut device);
        dev_send.send(encoder).unwrap();
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
