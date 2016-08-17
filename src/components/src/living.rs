#[derive(Debug)]
pub struct Living {
    state: State,
}

impl ::specs::Component for Living {
    type Storage = ::specs::VecStorage<Living>;
}

impl Living {
    pub fn new() -> Living {
        Living {
            state: State::Idle,
        }
    }

    pub fn walk(&mut self, direction: ::math::Point2) {
        self.state = State::Walking(direction);
    }

    pub fn fall(&mut self, speed: ::utils::Coord) {
        self.state = State::Falling(speed);
    }

    pub fn get_state(&self) -> State {
        self.state.clone()
    }
}

#[derive(Debug, Clone)]
pub enum State {
    Idle,
    Walking(::math::Point2),
    Falling(::utils::Coord),
}
