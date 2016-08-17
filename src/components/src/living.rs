#[derive(Debug)]
pub struct Component {
    state: State,
}

impl ::specs::Component for Component {
    type Storage = ::specs::VecStorage<Component>;
}

impl Component {
    pub fn new() -> Component {
        Component {
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
