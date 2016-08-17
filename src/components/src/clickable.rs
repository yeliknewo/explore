#[derive(Debug)]
pub struct Component {
    pub clicked: bool,
    pub hitbox: ::math::Rect,
}

impl ::specs::Component for Component {
    type Storage = ::specs::VecStorage<Component>;
}

impl Component {
    pub fn new(hitbox: ::math::Rect) -> Component {
        Component {
            clicked: false,
            hitbox: hitbox,
        }
    }
}
