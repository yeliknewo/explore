use ::gfx::Factory;

pub type Index = u32;

pub fn make_shaders() -> Result<::Shaders, ::utils::Error> {
    ::Shaders::new("texture_150_v.glsl", "texture_150_f.glsl")
}

gfx_defines! {
    vertex Vertex {
        pos: [f32; 3] = "a_Pos",
        uv: [f32; 2] = "a_Uv",
    }

    constant TextureData {
        tint: [f32; 4] = "u_Tint",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),

        projection_cb: gfx::ConstantBuffer<::ProjectionData> = "b_ProjData",

        texture: gfx::TextureSampler<[f32; 4]> = "t_Texture",

        texture_data: gfx::ConstantBuffer<TextureData> = "b_TextureData",

        out_color: ::gfx::BlendTarget<::ColorFormat> = ("Target0", gfx::state::MASK_ALL, gfx::preset::blend::ALPHA),
        out_depth: ::gfx::DepthTarget<::DepthFormat> = ::gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

impl Vertex {
    pub fn new(pos: [f32; 3], uv: [f32; 2]) -> Vertex {
        Vertex {
            pos: pos,
            uv: uv,
        }
    }
}

pub fn load_texture<P>(factory: &mut ::gfx_device_gl::Factory, path: P) -> Result<::gfx::handle::ShaderResourceView<::gfx_device_gl::Resources, [f32; 4]>, ::utils::Error>
where P: AsRef<::std::path::Path>
{
    let image = match ::image::open(path) {
        Ok(image) => image,
        Err(err) => {
            error!("image load error: {}", err);
            return Err(::utils::Error::Logged);
        },
    }.to_rgba();
    let (width, height) = image.dimensions();
    let kind = ::gfx::tex::Kind::D2(width as ::gfx::tex::Size, height as ::gfx::tex::Size, ::gfx::tex::AaMode::Single);
    let (_, view) = match factory.create_texture_const_u8::<::ColorFormat>(kind, &[&image]) {
        Ok(data) => data,
        Err(err) => {
            error!("factory create texture const error: {}", err);
            return Err(::utils::Error::Logged);
        },
    };
    Ok(view)
}

pub struct Bundle {
    slice: ::gfx::Slice<::gfx_device_gl::Resources>,
    pso: ::gfx::PipelineState<::gfx_device_gl::Resources, pipe::Meta>,
    pub data: pipe::Data<::gfx_device_gl::Resources>,
}

impl Bundle {
    pub fn new(
        slice: ::gfx::Slice<::gfx_device_gl::Resources>,
        pso: ::gfx::PipelineState<::gfx_device_gl::Resources, pipe::Meta>,
        data: pipe::Data<::gfx_device_gl::Resources>,
    ) -> Bundle
    {
        Bundle {
            slice: slice,
            pso: pso,
            data: data,
        }
    }

    pub fn encode(&self, encoder: &mut ::gfx::Encoder<::gfx_device_gl::Resources, ::gfx_device_gl::CommandBuffer>) {
        encoder.draw(&self.slice, &self.pso, &self.data);
    }
}

#[derive(Debug)]
pub struct Packet {
    vertices: Vec<Vertex>,
    indices: Vec<Index>,
    rasterizer: ::gfx::state::Rasterizer,
    texture: Option<::gfx::handle::ShaderResourceView<::gfx_device_gl::Resources, [f32; 4]>>,
}

impl Packet {
    pub fn new(
        vertices: Vec<Vertex>,
        indices: Vec<Index>,
        rasterizer: ::gfx::state::Rasterizer,
        texture: ::gfx::handle::ShaderResourceView<::gfx_device_gl::Resources, [f32; 4]>
    ) -> Packet {
        Packet {
            vertices: vertices,
            indices: indices,
            rasterizer: rasterizer,
            texture: Some(texture),
        }
    }

    pub fn get_vertices(&self) -> &[Vertex] {
        self.vertices.as_slice()
    }

    pub fn get_indices(&self) -> &[Index] {
        self.indices.as_slice()
    }

    pub fn get_rasterizer(&self) -> ::gfx::state::Rasterizer {
        self.rasterizer
    }

    pub fn get_texture(&mut self) -> ::gfx::handle::ShaderResourceView<::gfx_device_gl::Resources, [f32; 4]> {
        self.texture.take().unwrap()
    }
}
