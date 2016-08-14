use std::sync::mpsc;
use glutin;

pub type ReceiverHub = (
    mpsc::Receiver<::control::Event>,
    u8
);

pub struct SenderHub {
    control: mpsc::Sender<::control::Event>,
}

impl SenderHub {
    pub fn new() -> (SenderHub, ReceiverHub) {
        // let (s###, r###) = mpsc::channel();

        let (sc, rc) = mpsc::channel();

        (SenderHub {
            control: sc,
        }, (rc, 0))
    }

    pub fn process_glutin(&self, event: glutin::Event) {
        use glutin::Event::{Resized, KeyboardInput};
        use glutin::{ElementState, VirtualKeyCode};

        match event {
            // KeyboardInput(state, _, Some(VirtualKeyCode::A)) => self.####.send().unwrap(),
            KeyboardInput(state, _, Some(VirtualKeyCode::D)) |
            KeyboardInput(state, _, Some(VirtualKeyCode::Right)) => match state {
                ElementState::Pressed => self.control.send(::control::Event::Right(true)).unwrap(),
                ElementState::Released => self.control.send(::control::Event::Right(false)).unwrap(),
            },
            KeyboardInput(state, _, Some(VirtualKeyCode::A)) |
            KeyboardInput(state, _, Some(VirtualKeyCode::Left)) => match state {
                ElementState::Pressed => self.control.send(::control::Event::Left(true)).unwrap(),
                ElementState::Released => self.control.send(::control::Event::Left(false)).unwrap(),
            },
            KeyboardInput(state, _, Some(VirtualKeyCode::W)) |
            KeyboardInput(state, _, Some(VirtualKeyCode::Up)) => match state {
                ElementState::Pressed => self.control.send(::control::Event::Up(true)).unwrap(),
                ElementState::Released => self.control.send(::control::Event::Up(false)).unwrap(),
            },
            KeyboardInput(state, _, Some(VirtualKeyCode::S)) |
            KeyboardInput(state, _, Some(VirtualKeyCode::Down)) => match state {
                ElementState::Pressed => self.control.send(::control::Event::Down(true)).unwrap(),
                ElementState::Released => self.control.send(::control::Event::Down(false)).unwrap(),
            },
            Resized(width, height) => self.control.send(::control::Event::Resize(width, height)).unwrap(),
            _ => (),
        }
    }
}
