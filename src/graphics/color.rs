use gfx;
use gfx_device_gl;

pub type Index = u32;

pub const VERTEX_SHADER: &'static [u8] = include_bytes!("shader/color_150_v.glsl");
pub const FRAGMENT_SHADER: &'static [u8] = include_bytes!("shader/color_150_f.glsl");

gfx_defines! {
    vertex Vertex {
        pos: [f32; 3] = "a_Pos",
        color: [f32; 4] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),

        projection_cb: gfx::ConstantBuffer<::graphics::ProjectionData> = "b_ProjData",

        out_color: gfx::RenderTarget<::graphics::ColorFormat> = "Target0",
        out_depth: gfx::DepthTarget<::graphics::DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
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
    pub slice: gfx::Slice<gfx_device_gl::Resources>,
    pub pso: gfx::PipelineState<gfx_device_gl::Resources, pipe::Meta>,
    pub data: pipe::Data<gfx_device_gl::Resources>,
}

impl Bundle {
    pub fn new(
        slice: gfx::Slice<gfx_device_gl::Resources>,
        pso: gfx::PipelineState<gfx_device_gl::Resources, pipe::Meta>,
        data: pipe::Data<gfx_device_gl::Resources>
    ) -> Bundle
    {
        Bundle {
            slice: slice,
            pso: pso,
            data: data,
        }
    }

    pub fn encode(&self, encoder: &mut gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>) {
        encoder.draw(&self.slice, &self.pso, &self.data);
    }
}
