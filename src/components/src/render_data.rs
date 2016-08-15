pub struct RenderData {
    texture_data: Option<TextureData>,
}

impl ::specs::Component for RenderData {
    type Storage = ::specs::VecStorage<RenderData>;
}

impl RenderData {
    pub fn new_texture(tint: [f32; 4]) -> RenderData {
        RenderData {
            texture_data: Some(TextureData {
                tint: tint,
            }),
        }
    }

    pub fn set_tint(&mut self, tint: [f32; 4]) {
        self.texture_data.as_mut().unwrap().tint = tint;
    }

    pub fn get_tint(&self) -> [f32; 4] {
        self.texture_data.as_ref().unwrap().tint.clone()
    }
}

pub struct TextureData {
    pub tint: [f32; 4],
}
