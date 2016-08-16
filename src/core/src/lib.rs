extern crate gfx;
extern crate gfx_device_gl;
extern crate glutin;
extern crate gfx_window_glutin;
extern crate specs;
extern crate nalgebra;
extern crate time;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate art;
extern crate systems as sys;
extern crate utils;
extern crate components as comps;
extern crate graphics;
extern crate math;

pub mod event;
pub mod game;

pub fn start<F>(setup: F) -> Result<(), ::utils::Error>
where F: for<'a> Fn(&'a mut specs::Planner<::utils::Delta>, &'a mut sys::render::System, &'a mut gfx_device_gl::Factory) -> Result<(), ::utils::Error>
{
    let ((mut out_color, mut out_depth), mut factory, encoder, window, mut device) = ::graphics::build_graphics(640, 480);

    let (mut event_dev, game_event) = ::event::DevEventHub::new();

    event_dev.send_to_render(sys::render::RecvEvent::GraphicsData(out_color.clone(), out_depth.clone()));

    event_dev.send_to_render(sys::render::RecvEvent::Encoder(encoder.clone_empty()));
    event_dev.send_to_render(sys::render::RecvEvent::Encoder(encoder));

    let game = try!(game::Game::new(&mut factory, game_event, ::math::Point2::new(out_color.get_dimensions().0 as f32, out_color.get_dimensions().1 as f32), setup));

    std::thread::spawn(|| {
        let mut game = game;
        while game.frame() {}
    });

    'main: loop {
        match try!(event_dev.recv_from_render()) {
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
                match window.swap_buffers() {
                    Ok(()) => (),
                    Err(err) => {
                        error!("window swap buffers error: {}", err);
                        return Err(::utils::Error::Logged);
                    },
                };
                device.cleanup();
            },
            sys::render::SendEvent::Error(err) => return Err(err),
        }

        match event_dev.try_recv_from_control() {
            Ok(event) => match event {
                sys::control::SendEvent::Resize => {
                    gfx_window_glutin::update_views(&window, &mut out_color, &mut out_depth);
                    event_dev.send_to_render(sys::render::RecvEvent::GraphicsData(out_color.clone(), out_depth.clone()));
                },
            },
            Err(::utils::Error::Empty) => (),
            Err(::utils::Error::Logged) => return Err(::utils::Error::Logged),
        }
    }

    event_dev.send_to_render(sys::render::RecvEvent::Exit);
    event_dev.send_to_control(sys::control::RecvEvent::Exit);
    event_dev.send_to_game(game::RecvEvent::Exit);

    Ok(())
}
