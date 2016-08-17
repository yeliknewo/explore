#[derive(Debug)]
pub struct Component {
    tint: [f32; 4],
    dirty: bool,
    dirty_2: bool, // required because double buffering
}

impl ::specs::Component for Component {
    type Storage = ::specs::VecStorage<Component>;
}

impl Component {
    pub fn new(tint: [f32; 4]) -> Component {
        Component {
            tint: tint,
            dirty: true,
            dirty_2: true,
        }
    }

    pub fn set_tint(&mut self, tint: [f32; 4]) {
        self.tint = tint;
        self.set_dirty();
    }

    pub fn get_tint(&self) -> [f32; 4] {
        self.tint.clone()
    }

    fn set_dirty(&mut self) {
        self.dirty = true;
        self.dirty_2 = true;
    }

    pub fn take_dirty(&mut self) -> bool {
        let temp = self.dirty | self.dirty_2;
        self.dirty = false;
        if self.dirty {
            self.dirty_2 = false;
        }
        temp
    }
}
