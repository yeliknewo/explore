pub type Index = u32;

pub fn make_shaders() -> ::Shaders {
    ::Shaders::new("color_150_v.glsl", "color_150_f.glsl")
}

gfx_defines! {
    vertex Vertex {
        pos: [f32; 3] = "a_Pos",
        color: [f32; 4] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),

        projection_cb: gfx::ConstantBuffer<::ProjectionData> = "b_ProjData",

        out_color: gfx::RenderTarget<::ColorFormat> = "Target0",
        out_depth: gfx::DepthTarget<::DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

impl Vertex {
    pub fn new(pos: [f32; 3], color: [f32; 4]) -> Vertex {
        Vertex {
            pos: pos,
            color: color,
        }
    }
}

pub struct Bundle {
    pub slice: ::gfx::Slice<::gfx_device_gl::Resources>,
    pub pso: ::gfx::PipelineState<::gfx_device_gl::Resources, pipe::Meta>,
    pub data: pipe::Data<::gfx_device_gl::Resources>,
}

impl Bundle {
    pub fn new(
        slice: ::gfx::Slice<::gfx_device_gl::Resources>,
        pso: ::gfx::PipelineState<::gfx_device_gl::Resources, pipe::Meta>,
        data: pipe::Data<::gfx_device_gl::Resources>
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

pub struct Packet {
    vertices: Vec<Vertex>,
    indices: Vec<Index>,
    rasterizer: ::gfx::state::Rasterizer,
}

impl Packet {
    pub fn new(
        vertices: Vec<Vertex>,
        indices: Vec<Index>,
        rasterizer: ::gfx::state::Rasterizer
    ) -> Packet {
        Packet {
            vertices: vertices,
            indices: indices,
            rasterizer: rasterizer,
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
}
