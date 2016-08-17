#[derive(Debug)]
pub struct Component {
    textures: Vec<::gfx::handle::ShaderResourceView<::gfx_device_gl::Resources, [f32; 4]>>,
}

impl ::specs::Component for Component {
    type Storage = ::specs::VecStorage<Component>;
}

impl Component {
    pub fn new() -> Component {
        Component {
            textures: vec!(),
        }
    }

    pub fn get_mut_textures(&mut self) -> &mut Vec<::gfx::handle::ShaderResourceView<::gfx_device_gl::Resources, [f32; 4]>> {
        &mut self.textures
    }

    pub fn get_texture(&self, index: usize) -> Option<&::gfx::handle::ShaderResourceView<::gfx_device_gl::Resources, [f32; 4]>> {
        self.textures.get(index)
    }
}
