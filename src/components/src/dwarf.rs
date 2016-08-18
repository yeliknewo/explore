#[derive(Debug)]
pub struct Component {
    target_tile: ::math::Point2,
    speed: ::math::Float,
}

impl ::specs::Component for Component {
    type Storage = ::specs::VecStorage<Component>;
}

impl Component {
    pub fn new(speed: ::math::Float) -> Component {
        Component {
            target_tile: ::math::Point2::zero(),
            speed: speed,
        }
    }

    pub fn get_target_tile(&self) -> ::math::Point2 {
        self.target_tile.clone()
    }

    pub fn get_speed(&self) -> ::math::Float {
        self.speed
    }

    pub fn get_mut_target_tile(&mut self) -> &mut ::math::Point2 {
        &mut self.target_tile
    }
}
