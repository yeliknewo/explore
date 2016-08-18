#[derive(Debug)]
pub struct Component {
    state: State,
    last_state: State,
    walking_rects: Vec<[f32; 4]>,
    falling_rects: Vec<[f32; 4]>,
    frame_count: usize,
}

impl ::specs::Component for Component {
    type Storage = ::specs::VecStorage<Component>;
}

impl Component {
    pub fn new(walking_rects: Vec<[f32; 4]>, falling_rects: Vec<[f32; 4]>) -> Component {
        Component {
            state: State::Idle,
            last_state: State::Idle,
            walking_rects: walking_rects,
            falling_rects: falling_rects,
            frame_count: 0,
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

    pub fn get_walking_rects(&self) -> &[[f32; 4]] {
        self.walking_rects.as_slice()
    }

    pub fn get_falling_rects(&self) -> &[[f32; 4]] {
        self.falling_rects.as_slice()
    }

    fn cycle_anim(&mut self, rect_length: usize) {
        self.frame_count += 1;
        if self.frame_count >= rect_length {
            self.frame_count = 0;
        }
    }

    pub fn get_next_walking(&mut self) -> &[f32; 4] {
        let len = self.walking_rects.len();
        self.cycle_anim(len);
        match self.walking_rects.get(self.frame_count) {
            Some(rect) => rect,
            None => ::art::spritesheet::ERROR,
        }
    }

    pub fn get_next_falling(&mut self) -> &[f32; 4] {
        let len = self.falling_rects.len();
        self.cycle_anim(len);
        match self.falling_rects.get(self.frame_count) {
            Some(rect) => rect,
            None => ::art::spritesheet::ERROR,
        }
    }

    pub fn get_state(&self) -> State {
        self.state.clone()
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
