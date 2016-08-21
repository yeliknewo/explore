#[derive(Debug)]
pub struct Component {
    tiles: ::std::collections::HashMap<::math::Point2I, ::specs::Entity>,
    dirty: bool,
}

impl Component {
    pub fn new() -> Component {
        Component {
            tiles: ::std::collections::HashMap::new(),
            dirty: true,
        }
    }

    fn make_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn make_clean(&mut self) {
        self.dirty = false;
    }

    pub fn get_dirty(&self) -> bool {
        self.dirty
    }

    pub fn get_tile(&self, location: &::math::Point2I) -> Option<&::specs::Entity> {
        self.tiles.get(location)
    }

    pub fn get_nearest_tile(&self, location: &::math::Point2) -> Option<&::specs::Entity> {
        self.tiles.get(&location.clone().into())
    }

    pub fn get_mut_tiles(&mut self) -> &mut ::std::collections::HashMap<::math::Point2I, ::specs::Entity> {
        self.make_dirty();
        &mut self.tiles
    }
}

impl ::specs::Component for Component {
    type Storage = ::specs::VecStorage<Component>;
}
