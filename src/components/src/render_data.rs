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
        match self.texture_data.as_mut() {
            Some(texture_data) => texture_data,
            None => {
                error!("texture data was none");
                return;
            }
        }.tint = tint;
    }

    pub fn get_tint(&self) -> Result<[f32; 4], ::utils::Error> {
        Ok(match self.texture_data.as_ref() {
            Some(texture_data) => texture_data,
            None => {
                error!("texture data was none");
                return Err(::utils::Error::Logged);
            }
        }.tint.clone())
    }
}

pub struct TextureData {
    pub tint: [f32; 4],
}
