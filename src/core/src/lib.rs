extern crate gfx;
extern crate gfx_device_gl;
extern crate glutin;
extern crate gfx_window_glutin;
extern crate specs;
extern crate nalgebra;
extern crate time;

extern crate art;
extern crate systems as sys;
extern crate utils;
extern crate components as comps;
extern crate graphics;
extern crate math;

pub mod event;
pub mod game;

pub fn start<F>(setup: F)
where F: for<'a> Fn(&'a mut specs::Planner<::utils::Delta>, &'a mut sys::render::System, &'a mut gfx_device_gl::Factory)
{
    let ((mut out_color, mut out_depth), mut factory, encoder, window, mut device) = ::graphics::build_graphics(640, 480);

    let (mut event_dev, game_event) = ::event::DevEventHub::new();

    event_dev.send_to_render(sys::render::RecvEvent::GraphicsData(out_color.clone(), out_depth.clone()));

    event_dev.send_to_render(sys::render::RecvEvent::Encoder(encoder.clone_empty()));
    event_dev.send_to_render(sys::render::RecvEvent::Encoder(encoder));

    let game = game::Game::new(&mut factory, game_event, ::math::Point2::new(out_color.get_dimensions().0 as f32, out_color.get_dimensions().1 as f32), setup);

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
    event_dev.send_to_game(game::RecvEvent::Exit);
}
