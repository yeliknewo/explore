#[derive(Debug)]
pub struct GameEventHub {
    pub control_channel: Option<::sys::control::Channel>,
    pub render_channel: Option<::sys::render::Channel>,
    pub game_channel: Option<::game::Channel>,
    pub tile_builder_channel: Option<::sys::tile_builder::Channel>,
}

impl GameEventHub {
    pub fn new(
        control_channel: ::sys::control::Channel,
        render_channel: ::sys::render::Channel,
        game_channel: ::game::Channel,
        tile_builder_channel: ::sys::tile_builder::Channel,
    ) -> GameEventHub {
        GameEventHub {
            control_channel: Some(control_channel),
            render_channel: Some(render_channel),
            game_channel: Some(game_channel),
            tile_builder_channel: Some(tile_builder_channel),
        }
    }
}

#[derive(Debug)]
pub struct DevEventHub {
    send_to_control: ::std::sync::mpsc::Sender<::sys::control::RecvEvent>,
    recv_from_control: ::std::sync::mpsc::Receiver<::sys::control::SendEvent>,
    send_to_render: ::std::sync::mpsc::Sender<::sys::render::RecvEvent>,
    recv_from_render: ::std::sync::mpsc::Receiver<::sys::render::SendEvent>,
    send_to_game: ::std::sync::mpsc::Sender<::game::RecvEvent>,
    recv_from_game: ::std::sync::mpsc::Receiver<::game::SendEvent>,
    send_to_tile_builder: ::std::sync::mpsc::Sender<::sys::tile_builder::RecvEvent>,
    recv_from_tile_builder: ::std::sync::mpsc::Receiver<::sys::tile_builder::SendEvent>,
}

impl DevEventHub{
    pub fn new() -> (DevEventHub, GameEventHub) {

        let (send_to_control, recv_to_control) = ::std::sync::mpsc::channel();
        let (send_from_control, recv_from_control) = ::std::sync::mpsc::channel();
        let (send_to_render, recv_to_render) = ::std::sync::mpsc::channel();
        let (send_from_render, recv_from_render) = ::std::sync::mpsc::channel();
        let (send_to_game, recv_to_game) = ::std::sync::mpsc::channel();
        let (send_from_game, recv_from_game) = ::std::sync::mpsc::channel();
        let (send_to_tile_builder, recv_to_tile_builder) = ::std::sync::mpsc::channel();
        let (send_from_tile_builder, recv_from_tile_builder) = ::std::sync::mpsc::channel();

        (
            DevEventHub::new_internal(
                send_to_control, recv_from_control,
                send_to_render, recv_from_render,
                send_to_game, recv_from_game,
                send_to_tile_builder, recv_from_tile_builder
            ),
            GameEventHub::new((send_from_control, recv_to_control), (send_from_render, recv_to_render), (send_from_game, recv_to_game), (send_from_tile_builder, recv_to_tile_builder))
        )
    }

    fn new_internal(
        send_to_control: ::std::sync::mpsc::Sender<::sys::control::RecvEvent>,
        recv_from_control: ::std::sync::mpsc::Receiver<::sys::control::SendEvent>,
        send_to_render: ::std::sync::mpsc::Sender<::sys::render::RecvEvent>,
        recv_from_render: ::std::sync::mpsc::Receiver<::sys::render::SendEvent>,
        send_to_game: ::std::sync::mpsc::Sender<::game::RecvEvent>,
        recv_from_game: ::std::sync::mpsc::Receiver<::game::SendEvent>,
        send_to_tile_builder: ::std::sync::mpsc::Sender<::sys::tile_builder::RecvEvent>,
        recv_from_tile_builder: ::std::sync::mpsc::Receiver<::sys::tile_builder::SendEvent>,
    ) -> DevEventHub
    {
        DevEventHub {
            send_to_control: send_to_control,
            recv_from_control: recv_from_control,
            send_to_render: send_to_render,
            recv_from_render: recv_from_render,
            send_to_game: send_to_game,
            recv_from_game: recv_from_game,
            send_to_tile_builder: send_to_tile_builder,
            recv_from_tile_builder: recv_from_tile_builder,

        }
    }

    pub fn send_to_control(&mut self, event: ::sys::control::RecvEvent) {
        match self.send_to_control.send(event) {
            Ok(()) => (),
            Err(err) => error!("send to control error: {}", err),
        }
    }

    pub fn recv_from_control(&mut self) -> Result<::sys::control::SendEvent, ::utils::Error> {
        match self.recv_from_control.recv() {
            Ok(event) => Ok(event),
            Err(err) => {
                error!("recv from control error: {}", err);
                Err(::utils::Error::Logged)
            },
        }
    }

    pub fn try_recv_from_control(&mut self) -> Result<::sys::control::SendEvent, ::utils::Error> {
        match self.recv_from_control.try_recv() {
            Ok(event) => Ok(event),
            Err(err) => match err {
                ::std::sync::mpsc::TryRecvError::Empty => Err(::utils::Error::Empty),
                ::std::sync::mpsc::TryRecvError::Disconnected => {
                    error!("try recv from control was disconnected");
                    Err(::utils::Error::Logged)
                },
            }
        }
    }

    pub fn send_to_render(&mut self, event: ::sys::render::RecvEvent) {
        match self.send_to_render.send(event) {
            Ok(()) => (),
            Err(err) => error!("send to render error: {}", err),
        }
    }

    pub fn recv_from_render(&mut self) -> Result<::sys::render::SendEvent, ::utils::Error> {
        match self.recv_from_render.recv() {
            Ok(event) => Ok(event),
            Err(err) => {
                error!("recv from render err: {}", err);
                Err(::utils::Error::Logged)
            },
        }
    }

    pub fn send_to_game(&mut self, event: ::game::RecvEvent) {
        match self.send_to_game.send(event) {
            Ok(()) => (),
            Err(err) => error!("send to game error: {}", err),
        }
    }

    pub fn recv_from_game(&mut self) -> Result<::game::SendEvent, ::utils::Error> {
        match self.recv_from_game.recv() {
            Ok(event) => Ok(event),
            Err(err) => {
                error!("recv from game err: {}", err);
                Err(::utils::Error::Logged)
            }
        }
    }

    pub fn try_recv_from_game(&mut self) -> Result<::game::SendEvent, ::utils::Error> {
        match self.recv_from_game.try_recv() {
            Ok(event) => Ok(event),
            Err(err) => match err {
                ::std::sync::mpsc::TryRecvError::Empty => Err(::utils::Error::Empty),
                ::std::sync::mpsc::TryRecvError::Disconnected => {
                    error!("try recv from game was disconnected");
                    Err(::utils::Error::Logged)
                },
            },
        }
    }

    pub fn send_to_tile_builder(&mut self, event: ::sys::tile_builder::RecvEvent) {
        match self.send_to_tile_builder.send(event) {
            Ok(()) => (),
            Err(err) => error!("send to tile builder error: {}", err),
        }
    }

    pub fn try_recv_from_tile_builder(&mut self) -> Result<::sys::tile_builder::SendEvent, ::utils::Error> {
        match self.recv_from_tile_builder.try_recv() {
            Ok(event) => Ok(event),
            Err(err) => match err {
                ::std::sync::mpsc::TryRecvError::Empty => Err(::utils::Error::Empty),
                ::std::sync::mpsc::TryRecvError::Disconnected => {
                    error!("try recv from tile builder was disconnected");
                    Err(::utils::Error::Logged)
                },
            },
        }
    }

    pub fn process_glutin(&mut self, event: ::glutin::Event) {
        match event {
            ::glutin::Event::MouseMoved(x, y) => self.send_to_control(::sys::control::RecvEvent::MouseMoved(x as u32, y as u32)),
            ::glutin::Event::MouseInput(state, button) => self.send_to_control(::sys::control::RecvEvent::MouseInput(match state {
                ::glutin::ElementState::Pressed => true,
                ::glutin::ElementState::Released => false,
            },
            button)),
            ::glutin::Event::KeyboardInput(state, _, Some(::glutin::VirtualKeyCode::D)) |
            ::glutin::Event::KeyboardInput(state, _, Some(::glutin::VirtualKeyCode::Right)) => match state {
                ::glutin::ElementState::Pressed => self.send_to_control(::sys::control::RecvEvent::Right(true)),
                ::glutin::ElementState::Released => self.send_to_control(::sys::control::RecvEvent::Right(false)),
            },
            ::glutin::Event::KeyboardInput(state, _, Some(::glutin::VirtualKeyCode::A)) |
            ::glutin::Event::KeyboardInput(state, _, Some(::glutin::VirtualKeyCode::Left)) => match state {
                ::glutin::ElementState::Pressed => self.send_to_control(::sys::control::RecvEvent::Left(true)),
                ::glutin::ElementState::Released => self.send_to_control(::sys::control::RecvEvent::Left(false)),
            },
            ::glutin::Event::KeyboardInput(state, _, Some(::glutin::VirtualKeyCode::W)) |
            ::glutin::Event::KeyboardInput(state, _, Some(::glutin::VirtualKeyCode::Up)) => match state {
                ::glutin::ElementState::Pressed => self.send_to_control(::sys::control::RecvEvent::Up(true)),
                ::glutin::ElementState::Released => self.send_to_control(::sys::control::RecvEvent::Up(false)),
            },
            ::glutin::Event::KeyboardInput(state, _, Some(::glutin::VirtualKeyCode::S)) |
            ::glutin::Event::KeyboardInput(state, _, Some(::glutin::VirtualKeyCode::Down)) => match state {
                ::glutin::ElementState::Pressed => self.send_to_control(::sys::control::RecvEvent::Down(true)),
                ::glutin::ElementState::Released => self.send_to_control(::sys::control::RecvEvent::Down(false)),
            },
            ::glutin::Event::Resized(width, height) => self.send_to_control(::sys::control::RecvEvent::Resize(width, height)),
            _ => (),
        }
    }
}
