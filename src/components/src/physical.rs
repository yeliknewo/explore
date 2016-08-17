#[derive(Debug)]
pub struct Physical {
    speed: ::math::Point2,
    friction: ::math::Point2,
    speed_break: ::math::Point2,
}

impl ::specs::Component for Physical {
    type Storage = ::specs::VecStorage<Physical>;
}

impl Physical {
    pub fn new(
        speed: ::math::Point2,
        friction: ::math::Point2,
        speed_break: ::math::Point2
    ) -> Physical {
        Physical {
            speed: speed,
            friction: friction,
            speed_break: speed_break,
        }
    }

    pub fn new_zero() -> Physical {
        Physical::new(
            ::math::Point2::new(0.0, 0.0),
            ::math::Point2::new(1.0, 1.0),
            ::math::Point2::new(0.01, 0.01)
        )
    }

    pub fn get_mut_speed(&mut self) -> &mut ::math::Point2 {
        &mut self.speed
    }

    pub fn get_mut_friction(&mut self) -> &mut ::math::Point2 {
        &mut self.friction
    }

    pub fn get_mut_speed_break(&mut self) -> &mut ::math::Point2 {
        &mut self.speed_break
    }

    pub fn get_speed(&self) -> ::math::Point2 {
        self.speed.clone()
    }

    pub fn get_friction(&self) -> ::math::Point2 {
        self.friction.clone()
    }

    pub fn get_speed_break(&self) -> ::math::Point2 {
        self.speed_break.clone()
    }
}