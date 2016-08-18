#[derive(Debug)]
pub struct Component {
    target_tile: Option<::math::Point2>,
    speed: ::math::Float,
}

impl ::specs::Component for Component {
    type Storage = ::specs::VecStorage<Component>;
}

impl Component {
    pub fn new(speed: ::math::Float) -> Component {
        Component {
            target_tile: None,
            speed: speed,
        }
    }

    pub fn get_target_tile(&self) -> Option<&::math::Point2> {
        self.target_tile.as_ref()
    }

    pub fn get_speed(&self) -> ::math::Float {
        self.speed
    }

    pub fn get_mut_target_tile(&mut self) -> Option<&mut ::math::Point2> {
        self.target_tile.as_mut()
    }

    pub fn get_mut_target_tile_opt(&mut self) -> &mut Option<::math::Point2> {
        &mut self.target_tile
    }
}
