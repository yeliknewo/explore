#[derive(Copy, Clone, PartialEq, Hash)]
pub struct RenderType {
    pub id: usize,
    pub renderer_type: ::graphics::RendererType,
}

impl ::specs::Component for RenderType {
    type Storage = ::specs::VecStorage<RenderType>;
}
