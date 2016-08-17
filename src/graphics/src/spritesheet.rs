pub type Index = u32;

pub fn make_shaders() -> Result<::Shaders, ::utils::Error> {
    ::Shaders::new("texture_150_v.glsl", "texture_150_f.glsl")
}

gfx_defines! {
    vertex Vertex {
        pos: [f32; 3] = "a_Pos",
        buf_pos: [f32; 2] = "a_BufPos",
        uv: [f32; 2] = "a_Uv",
    }

    constant TextureData {
        tint: [f32; 4] = "u_Tint",
        tilesheet_step: [f32; 2] = "u_TilesheetStep",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),

        projection_cb: gfx::ConstantBuffer<::ProjectionData> = "b_ProjData",

        spritesheet: gfx::TextureSampler<[f32; 4]> = "t_Texture",

        texture_data: gfx::ConstantBuffer<TextureData> = "b_TextureData",

        out_color: ::gfx::BlendTarget<::ColorFormat> = ("Target0", gfx::state::MASK_ALL, gfx::preset::blend::ALPHA),
        out_depth: gfx::DepthTarget<::DepthFormat> = ::gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

impl Vertex {
    pub fn new(pos: [f32; 3], buf_pos: [f32; 2], uv: [f32; 2]) -> Vertex {
        Vertex {
            pos: pos,
            buf_pos: buf_pos,
            uv: uv,
        }
    }
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
    ) -> Bundle {
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