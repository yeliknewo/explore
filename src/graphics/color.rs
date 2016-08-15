pub type Index = u32;

const VERTEX_SHADER: &'static [u8] = include_bytes!("shader/color_150_v.glsl");
const FRAGMENT_SHADER: &'static [u8] = include_bytes!("shader/color_150_f.glsl");

gfx_defines! {
    constant ProjectionData {
        model: [[f32; 4]; 4] = "u_Model",
        view: [[f32; 4]; 4] = "u_View",
        proj: [[f32; 4]; 4] = "u_Proj",
    }

    vertex Vertex {
        pos: [f32; 3] = "a_Pos",
        color: [f32; 4] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),

        projection_cb: gfx::ConstantBuffer<ProjectionData> = "b_ProjData",

        out_color: gfx::RenderTarget<ColorFormat> = "Target0",
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
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

struct Bundle {
    slice: gfx::Slice<gfx_device_gl::Resources>,
    pso: gfx::PipelineState<gfx_device_gl::Resources, data::Meta>,
    data: Box<gfx::pso::PipelineData<gfx_device_gl::Resources>>,
}

impl Bundle {
    fn new(
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

    pub fn update_data(&mut self, out_color: gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>, out_depth: gfx::handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat>) {
        self.data.out_color = out_color;
        self.data.out_depth = out_depth;
    }
}
