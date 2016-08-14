use specs;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct RenderType(pub usize);

impl specs::Component for RenderType {
    type Storage = specs::VecStorage<RenderType>;
}
