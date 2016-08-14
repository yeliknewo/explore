use std::sync::{mpsc, Arc};
use gfx;
use gfx::traits::FactoryExt;
use gfx_device_gl;
use gfx_window_glutin;
use glutin;
use specs;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub type Index = u32;

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

pub fn build_graphics() -> (
    (gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>, gfx::handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat>),
    gfx_device_gl::Factory,
    gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
    glutin::Window,
    gfx_device_gl::Device
) {
    let builder = glutin::WindowBuilder::new()
        .with_title("Explore")
        .with_dimensions(1024, 768)
        .with_vsync()
    ;

    let (window, device, mut factory, out_color, out_depth) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    let encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    (
        (
            out_color,
            out_depth
        ),
        factory,
        encoder,
        window,
        device
    )
}

struct Bundle {
    slice: gfx::Slice<gfx_device_gl::Resources>,
    pso: gfx::PipelineState<gfx_device_gl::Resources, pipe::Meta>,
    data: pipe::Data<gfx_device_gl::Resources>,
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

pub type Channel = (
    mpsc::Sender<SendEvent>,
    mpsc::Receiver<RecvEvent>
);

pub enum SendEvent {
    Encoder(gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>),
}

pub enum RecvEvent {
    Encoder(gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>),
    GraphicsData(gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>, gfx::handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat>)
}

pub struct System {
    channel: Channel,
    out_color: gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>,
    out_depth: gfx::handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat>,
    bundles: Arc<Vec<Bundle>>,
}

impl System {
    pub fn new(
        channel: Channel
    ) -> System
    {
        let (out_color, out_depth) = match channel.1.recv().unwrap() {
            RecvEvent::GraphicsData(out_color, out_depth) => (out_color, out_depth),
            _ => panic!("render system received non graphics data first from channel"),
        };

        System {
            channel: channel,
            out_color: out_color,
            out_depth: out_depth,
            bundles: Arc::new(Vec::new()),
        }
    }

    pub fn add_render_type(&mut self,
        factory: &mut gfx_device_gl::Factory,
        vertices: &[Vertex],
        indices: &[Index]
    ) -> ::comps::RenderType
    {
        let pso = factory.create_pipeline_simple (
            include_bytes!("../shader/explore_150_v.glsl"),
            include_bytes!("../shader/explore_150_f.glsl"),
            pipe::new()
        ).unwrap();
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(vertices, indices);
        let data = pipe::Data {
            vbuf: vbuf,
            projection_cb: factory.create_constant_buffer(1),
            out_color: self.out_color.clone(),
            out_depth: self.out_depth.clone(),
        };
        let id = self.bundles.len();
        let mut bundles = Arc::get_mut(&mut self.bundles).unwrap();
        bundles.push(Bundle::new(slice, pso, data));
        ::comps::RenderType(id)
    }

    fn render(&mut self, arg: &specs::RunArg, mut encoder: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>) {
        use specs::Join;

        let (draw, transform, camera) = arg.fetch(|w| {
            (w.read::<::comps::RenderType>(), w.read::<::comps::Transform>(), w.read::<::comps::Camera>())
        });

        encoder.clear(&self.out_color, [0.1, 0.1, 0.1, 1.0]);
        encoder.clear_depth(&self.out_depth, 1.0);

        let (view, proj) = {
            let camera = {
                let mut camera_opt = None;

                for c in (&camera).iter() {
                    camera_opt = Some(c);
                }

                camera_opt.unwrap()
            };

            (camera.get_view(), camera.get_proj())
        };

        for (d, t) in (&draw, &transform).iter() {
            let projection_data = ProjectionData {
                model: t.get_model(),
                view: view,
                proj: proj,
            };
            let b = &self.bundles[d.0];
            encoder.update_constant_buffer(&b.data.projection_cb, &projection_data);
            b.encode(&mut encoder);
        }

        let _ = self.channel.0.send(SendEvent::Encoder(encoder));
    }

    fn set_graphics_data(&mut self, out_color: gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>, out_depth: gfx::handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat>) {
        self.out_color = out_color;
        self.out_depth = out_depth;

        for bundle in Arc::get_mut(&mut self.bundles).unwrap() {
            bundle.update_data(self.out_color.clone(), self.out_depth.clone());
        }
    }

    fn process_event(&mut self, arg: &specs::RunArg, event: RecvEvent) -> bool {
        match event {
            RecvEvent::Encoder(encoder) => {
                self.render(arg, encoder);
                false
            },
            RecvEvent::GraphicsData(out_color, out_depth) => {
                self.set_graphics_data(out_color, out_depth);
                true
            },
        }
    }
}

impl specs::System<::Delta> for System {
    fn run(&mut self, arg: specs::RunArg, _: ::Delta) {
        let mut event = self.channel.1.recv().unwrap();
        while self.process_event(&arg, event) {
            event = self.channel.1.recv().unwrap();
        }
    }
}
