#[derive(Debug)]
pub struct Clickable {
    pub clicked: bool,
    pub hitbox: ::math::Rect,
}

impl ::specs::Component for Clickable {
    type Storage = ::specs::VecStorage<Clickable>;
}

impl Clickable {
    pub fn new(hitbox: ::math::Rect) -> Clickable {
        Clickable {
            clicked: false,
            hitbox: hitbox,
        }
    }
}
