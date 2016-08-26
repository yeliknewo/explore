use std::collections::HashMap;

use math::Point2I;

#[derive(Debug)]
pub struct Component {
    tiles: ::std::collections::HashMap<::math::Point2I, ::specs::Entity>,
    placeheld_tiles: HashMap<Point2I, bool>,
    dirty: bool,
}

impl Component {
    pub fn new() -> Component {
        Component {
            tiles: ::std::collections::HashMap::new(),
            placeheld_tiles: HashMap::new(),
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

    pub fn hold_place(&mut self, location: &Point2I) {
        self.placeheld_tiles.insert(location.clone(), true);
    }

    pub fn is_tile_placeheld(&self, location: &Point2I) -> bool {
        match self.placeheld_tiles.get(location) {
            Some(value) => *value,
            None => false,
        }
    }
}

impl ::specs::Component for Component {
    type Storage = ::specs::VecStorage<Component>;
}
