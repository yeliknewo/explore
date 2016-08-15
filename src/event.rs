use std::sync::mpsc;
use glutin;

pub struct GameEventHub {
    pub control_channel: Option<::sys::control::Channel>,
    pub render_channel: Option<::sys::render::Channel>,
    pub game_channel: Option<::game::Channel>,
}

impl GameEventHub {
    pub fn new(
        control_channel: (mpsc::Sender<::sys::control::SendEvent>, mpsc::Receiver<::sys::control::RecvEvent>),
        render_channel: (mpsc::Sender<::sys::render::SendEvent>, mpsc::Receiver<::sys::render::RecvEvent>),
        game_channel: mpsc::Receiver<::game::RecvEvent>,
    ) -> GameEventHub {
        GameEventHub {
            control_channel: Some(control_channel),
            render_channel: Some(render_channel),
            game_channel: Some(game_channel),
        }
    }
}

pub struct DevEventHub {
    send_to_control: mpsc::Sender<::sys::control::RecvEvent>,
    recv_from_control: mpsc::Receiver<::sys::control::SendEvent>,
    send_to_render: mpsc::Sender<::sys::render::RecvEvent>,
    recv_from_render: mpsc::Receiver<::sys::render::SendEvent>,
    send_to_game: mpsc::Sender<::game::RecvEvent>,
}

impl DevEventHub{
    pub fn new() -> (DevEventHub, GameEventHub) {
        // let (s###, r###) = mpsc::channel();

        let (send_to_control, recv_to_control) = mpsc::channel();
        let (send_from_control, recv_from_control) = mpsc::channel();
        let (send_to_render, recv_to_render) = mpsc::channel();
        let (send_from_render, recv_from_render) = mpsc::channel();
        let (send_to_game, recv_to_game) = mpsc::channel();
        (
            DevEventHub::new_internal(
                send_to_control, recv_from_control,
                send_to_render, recv_from_render,
                send_to_game
            ),
            GameEventHub::new((send_from_control, recv_to_control), (send_from_render, recv_to_render), recv_to_game)
        )
    }

    fn new_internal(
        send_to_control: mpsc::Sender<::sys::control::RecvEvent>,
        recv_from_control: mpsc::Receiver<::sys::control::SendEvent>,
        send_to_render: mpsc::Sender<::sys::render::RecvEvent>,
        recv_from_render: mpsc::Receiver<::sys::render::SendEvent>,
        send_to_game: mpsc::Sender<::game::RecvEvent>,
    ) -> DevEventHub
    {
        DevEventHub {
            send_to_control: send_to_control,
            recv_from_control: recv_from_control,
            send_to_render: send_to_render,
            recv_from_render: recv_from_render,
            send_to_game: send_to_game,
        }
    }

    pub fn send_to_control(&mut self, event: ::sys::control::RecvEvent) {
        self.send_to_control.send(event).unwrap();
    }

    pub fn recv_from_control(&mut self) -> ::sys::control::SendEvent {
        self.recv_from_control.recv().unwrap()
    }

    pub fn try_recv_from_control(&mut self) -> Result<::sys::control::SendEvent, mpsc::TryRecvError> {
        self.recv_from_control.try_recv()
    }

    pub fn send_to_render(&mut self, event: ::sys::render::RecvEvent) {
        self.send_to_render.send(event).unwrap();
    }

    pub fn recv_from_render(&mut self) -> ::sys::render::SendEvent {
        self.recv_from_render.recv().unwrap()
    }

    pub fn send_to_game(&mut self, event: ::game::RecvEvent) {
        self.send_to_game.send(event).unwrap();
    }

    pub fn process_glutin(&mut self, event: glutin::Event) {
        use glutin::Event::{Resized, KeyboardInput, MouseMoved, MouseInput};
        use glutin::{ElementState, VirtualKeyCode};

        match event {
            MouseMoved(x, y) => self.send_to_control.send(::sys::control::RecvEvent::MouseMoved(x as u32, y as u32)).unwrap(),
            MouseInput(state, button) => self.send_to_control.send(::sys::control::RecvEvent::MouseInput(match state {
                ElementState::Pressed => true,
                ElementState::Released => false,
            },
            button)).unwrap(),
            KeyboardInput(state, _, Some(VirtualKeyCode::D)) |
            KeyboardInput(state, _, Some(VirtualKeyCode::Right)) => match state {
                ElementState::Pressed => self.send_to_control.send(::sys::control::RecvEvent::Right(true)).unwrap(),
                ElementState::Released => self.send_to_control.send(::sys::control::RecvEvent::Right(false)).unwrap(),
            },
            KeyboardInput(state, _, Some(VirtualKeyCode::A)) |
            KeyboardInput(state, _, Some(VirtualKeyCode::Left)) => match state {
                ElementState::Pressed => self.send_to_control.send(::sys::control::RecvEvent::Left(true)).unwrap(),
                ElementState::Released => self.send_to_control.send(::sys::control::RecvEvent::Left(false)).unwrap(),
            },
            KeyboardInput(state, _, Some(VirtualKeyCode::W)) |
            KeyboardInput(state, _, Some(VirtualKeyCode::Up)) => match state {
                ElementState::Pressed => self.send_to_control.send(::sys::control::RecvEvent::Up(true)).unwrap(),
                ElementState::Released => self.send_to_control.send(::sys::control::RecvEvent::Up(false)).unwrap(),
            },
            KeyboardInput(state, _, Some(VirtualKeyCode::S)) |
            KeyboardInput(state, _, Some(VirtualKeyCode::Down)) => match state {
                ElementState::Pressed => self.send_to_control.send(::sys::control::RecvEvent::Down(true)).unwrap(),
                ElementState::Released => self.send_to_control.send(::sys::control::RecvEvent::Down(false)).unwrap(),
            },
            Resized(width, height) => self.send_to_control.send(::sys::control::RecvEvent::Resize(width, height)).unwrap(),
            _ => (),
        }
    }
}
