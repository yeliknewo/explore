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
        use glutin::Event::KeyboardInput;
        use glutin::{ElementState, VirtualKeyCode};

        match event {
            // KeyboardInput(state, _, Some(VirtualKeyCode::A)) => self.####.send().unwrap(),
            _ => (),
        }
    }
}
