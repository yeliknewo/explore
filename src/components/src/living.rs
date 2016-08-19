pub struct Component {
    future_state_pairs: Vec<(State, StateData)>,
    state_pair: (State, StateData),
    last_state_pair: (State, StateData),
    state_rects_map: ::std::collections::HashMap<State, Vec<[f32; 4]>>,
    frame_count: usize,
}

impl ::specs::Component for Component {
    type Storage = ::specs::VecStorage<Component>;
}

impl Component {
    pub fn new(idle_rects: Vec<[f32; 4]>, walking_rects: Vec<[f32; 4]>, falling_rects: Vec<[f32; 4]>) -> Component {
        let mut rects = ::std::collections::HashMap::new();

        rects.insert(State::Idle, idle_rects);
        rects.insert(State::Walking, walking_rects);
        rects.insert(State::Falling, falling_rects);

        Component {
            future_state_pairs: vec!(),
            state_pair: (State::Idle, StateData::Idle),
            last_state_pair: (State::Idle, StateData::Idle),
            state_rects_map: rects,
            frame_count: 0,

        }
    }

    fn set_future_state_pair(&mut self, state_pair: (State, StateData)) {
        self.future_state_pairs.push(state_pair);
    }

    fn set_state_pair(&mut self, new_state_pair: (State, StateData)) {
        if self.last_state_pair.0 != new_state_pair.0 {
            self.frame_count = 0;
        }
        self.last_state_pair = self.state_pair.clone();
        self.state_pair = new_state_pair;
    }

    pub fn update_state(&mut self) {
        match self.future_state_pairs.pop() {
            Some(state_pair) => self.set_state_pair(state_pair),
            None => (),
        }

        self.frame_count += 1;
        if self.frame_count >= match self.state_rects_map.get(&self.state_pair.0) {
            Some(vec) => vec.len(),
            None => {
                error!("state rects map vec was none for state: {}", self.state_pair.0);
                0
            }
        }{
            self.frame_count = 0;
        }
    }

    pub fn idle(&mut self) {
        self.set_future_state_pair((State::Idle, StateData::Idle));
    }

    pub fn walk(&mut self, direction: ::math::Point2) {
        self.set_future_state_pair((State::Walking, StateData::Walking(direction)));
    }

    pub fn walk_to(&mut self, location: ::math::Point2) {
        self.set_future_state_pair((State::Walking, StateData::MoveTo(location)));
    }

    pub fn fall(&mut self, speed: ::utils::Coord) {
        self.set_future_state_pair((State::Falling, StateData::Falling(speed)));
    }

    pub fn get_next_rect(&self) -> &[f32; 4] {
        match self.state_rects_map.get(&self.state_pair.0) {
            Some(rect_vec) => match rect_vec.get(self.frame_count) {
                Some(rect) => rect,
                None => {
                    error!("rect vec had no rect for frame count: {}", self.frame_count);
                    ::art::spritesheet::ERROR
                },
            },
            None => {
                error!("state rects map had no vec for state: {}", self.state_pair.0);
                ::art::spritesheet::ERROR
            },
        }
    }

    pub fn get_state_pair(&self) -> &(State, StateData) {
        &self.state_pair
    }

    pub fn is_state_new(&self) -> bool {
        self.state_pair.0 == self.last_state_pair.0
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum StateData {
    Idle,
    Walking(::math::Point2),
    Falling(::utils::Coord),
    MoveTo(::math::Point2),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum State {
    Idle,
    Walking,
    Falling,
}

impl ::std::fmt::Display for State {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            State::Idle => write!(f, "State: Idle"),
            State::Walking => write!(f, "State: Walking"),
            State::Falling => write!(f, "State: Falling"),
        }
    }
}
