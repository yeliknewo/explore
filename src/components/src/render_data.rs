#[derive(Debug)]
pub struct Component {
    tint: [f32; 4],
    texture_index: Vec<usize>,
    texture_index_index: usize,
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
            texture_index: vec!(),
            texture_index_index: 0,
            dirty: true,
            dirty_2: true,
        }
    }

    pub fn set_tint(&mut self, tint: [f32; 4]) {
        self.tint = tint;
        self.set_dirty();
    }

    pub fn set_texture_index(&mut self, texture_index: Vec<usize>) {
        self.texture_index = texture_index;
        self.set_dirty();
    }

    pub fn get_texture_index_index(&mut self) -> usize {
        self.texture_index_index += 1;
        self.texture_index_index - 1
    }

    pub fn get_tint(&self) -> [f32; 4] {
        self.tint.clone()
    }

    pub fn get_texture_index(&self) -> &Vec<usize> {
        &self.texture_index
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
