#[derive(Debug)]
pub struct RenderData {
    texture_data: Option<TextureData>,
    dirty: bool,
    dirty_2: bool, // required because double buffering
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
            dirty: true,
            dirty_2: true,
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
        self.set_dirty();
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

    fn set_dirty(&mut self) {
        self.dirty = true;
        self.dirty_2 = true;
    }

    pub fn take_dirty(&mut self) -> bool {
        let temp = self.dirty | self.dirty_2;
        self.dirty = false;
        if self.dirty {
            self.dirty_2 = false;
        }
        temp
    }
}

#[derive(Debug)]
struct TextureData {
    tint: [f32; 4],
}
