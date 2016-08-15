#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate nalgebra;
extern crate specs;
extern crate time;
extern crate image;
extern crate find_folder;

extern crate systems as sys;
extern crate graphics;
extern crate core;
extern crate art;
extern crate components;
extern crate utils;

fn main() {
    let ((mut out_color, mut out_depth), mut factory, encoder, window, mut device) = graphics::build_graphics(640, 480);

    let (mut event_dev, game_event) = ::core::event::DevEventHub::new();

    event_dev.send_to_render(sys::render::RecvEvent::GraphicsData(out_color.clone(), out_depth.clone()));

    event_dev.send_to_render(sys::render::RecvEvent::Encoder(encoder.clone_empty()));
    event_dev.send_to_render(sys::render::RecvEvent::Encoder(encoder));

    let game = core::game::Game::new(&mut factory, game_event);

    std::thread::spawn(|| {
        let mut game = game;
        while game.frame() {}
    });

    'main: loop {
        match event_dev.recv_from_render() {
            sys::render::SendEvent::Encoder(mut encoder) => {
                use gfx::Device;

                for event in window.poll_events() {
                    match event {
                        glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                        glutin::Event::Closed => break 'main,
                        _ => event_dev.process_glutin(event),
                    }
                }

                encoder.flush(&mut device);
                event_dev.send_to_render(sys::render::RecvEvent::Encoder(encoder));
                window.swap_buffers().unwrap();
                device.cleanup();
            },
        }

        if let Ok(event) = event_dev.try_recv_from_control() {
            match event {
                sys::control::SendEvent::Resize => {
                    gfx_window_glutin::update_views(&window, &mut out_color, &mut out_depth);
                    event_dev.send_to_render(sys::render::RecvEvent::GraphicsData(out_color.clone(), out_depth.clone()));
                }
            }
        }
    }

    event_dev.send_to_render(sys::render::RecvEvent::Exit);
    event_dev.send_to_control(sys::control::RecvEvent::Exit);
    event_dev.send_to_game(core::game::RecvEvent::Exit);
}
