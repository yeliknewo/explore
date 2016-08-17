#[derive(Debug)]
pub struct Component {
    state: State,
    last_state: State,
    walking_index: Vec<usize>,
    falling_index: Vec<usize>,
}

impl ::specs::Component for Component {
    type Storage = ::specs::VecStorage<Component>;
}

impl Component {
    pub fn new(walking_index: Vec<usize>, falling_index: Vec<usize>) -> Component {
        Component {
            state: State::Idle,
            last_state: State::Idle,
            walking_index: walking_index,
            falling_index: falling_index,
        }
    }

    pub fn walk(&mut self, direction: ::math::Point2) {
        self.last_state = self.state.clone();
        self.state = State::Walking(direction);
    }

    pub fn fall(&mut self, speed: ::utils::Coord) {
        self.last_state = self.state.clone();
        self.state = State::Falling(speed);
    }

    pub fn get_state(&self) -> State {
        self.state.clone()
    }

    pub fn get_walking_index(&self) -> &Vec<usize> {
        &self.walking_index
    }

    pub fn get_falling_index(&self) -> &Vec<usize> {
        &self.falling_index
    }

    pub fn is_state_new(&self) -> bool {
        self.state == self.last_state
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum State {
    Idle,
    Walking(::math::Point2),
    Falling(::utils::Coord),
}
