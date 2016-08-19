#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub struct Component {
    pub id: usize,
}

impl ::specs::Component for Component {
    type Storage = ::specs::VecStorage<Component>;
}
