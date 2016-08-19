#[derive(Debug)]
pub struct Component {
    tiles: ::std::collections::HashMap<::math::Point2I, ::specs::Entity>,
}

impl Component {
    pub fn new() -> Component {
        Component {
            tiles: ::std::collections::HashMap::new(),
        }
    }

    pub fn get_tile(&self, location: &::math::Point2I) -> Option<&::specs::Entity> {
        self.tiles.get(location)
    }

    pub fn get_nearest_tile(&self, location: &::math::Point2) -> Option<&::specs::Entity> {
        self.tiles.get(&location.clone().into())
    }

    pub fn get_mut_tiles(&mut self) -> &mut ::std::collections::HashMap<::math::Point2I, ::specs::Entity> {
        &mut self.tiles
    }
}

impl ::specs::Component for Component {
    type Storage = ::specs::VecStorage<Component>;
}
