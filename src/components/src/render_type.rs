#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub struct Component {
    pub id: usize,
    pub renderer_type: ::graphics::RendererType,
}

impl ::specs::Component for Component {
    type Storage = ::specs::VecStorage<Component>;
}
