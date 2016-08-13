use std::sync::{mpsc, Arc};
use gfx_core;
// use gfx;
// use gfx::traits::FactoryExt;
// use gfx::{Device};
use gfx_device_gl;
use gfx_window_glutin;
use glutin;
use nalgebra;
use nalgebra::{ToHomogeneous};
use specs;

pub type ColorFormat = gfx_core::format::Rgba8;
pub type DepthFormat = gfx_core::format::DepthStencil;

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
    fn new(pos: [f32; 3], color: [f32; 4]) -> Vertex {
        Vertex {
            pos: pos,
            color: color,
        }
    }
}

pub struct Graphics<R: gfx_core::Resources, F: gfx_core::factory::Factory<R>> {
    out_color: gfx_core::handle::RenderTargetView<R, ColorFormat>,
    out_depth: gfx_core::handle::DepthStencilView<R, DepthFormat>,
    factory: F,
}

impl<R, F> Graphics<R, F> where
R: gfx_core::Resources,
F: gfx_core::factory::Factory<R>
{
    pub fn new_glutin() -> (Graphics<gfx_device_gl::Resources, gfx_device_gl::Factory>, gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>) {
        let builder = glutin::WindowBuilder::new()
            .with_title("Explore")
            .with_dimensions(1024, 768)
            .with_vsync();

        let (window, mut device, mut factory, main_color, main_depth) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

        let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

        (
            Graphics {
                out_color: main_color,
                out_depth: main_depth,
                factory: factory,
            },
            encoder
        )
    }

    pub fn make_render_system<C: gfx_core::draw::CommandBuffer<R>>(&self, encoder_channel: EncoderChannel<R, C>) -> RenderSystem<R, C> {
        RenderSystem::new(encoder_channel, self.out_color.clone(), self.out_depth.clone())
    }

    pub fn get_args(&mut self) -> &mut F {
        &mut self.factory
    }
}

// pub fn build() {
//     let builder = glutin::WindowBuilder::new()
//         .with_title("Explore")
//         .with_dimensions(1024, 768)
//         .with_vsync();
//
//     let (window, mut device, mut factory, main_color, main_depth) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);
//
//     let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
//
//     let pso = factory.create_pipeline_simple (
//         include_bytes!("shader/explore_150_v.glsl"),
//         include_bytes!("shader/explore_150_f.glsl"),
//         pipe::new()
//     ).unwrap_or_else(|e| panic!(e));
//
//     let vertex_data = &[
//         Vertex::new([-1.0, -1.0, -1.0], [0.0, 0.0, 0.0, 1.0]),
//         Vertex::new([-1.0, -1.0,  1.0], [0.0, 0.0, 1.0, 1.0]),
//         Vertex::new([-1.0,  1.0,  1.0], [0.0, 1.0, 0.0, 1.0]),
//         Vertex::new([-1.0,  1.0, -1.0], [0.0, 1.0, 1.0, 1.0]),
//
//         Vertex::new([ 1.0, -1.0, -1.0], [1.0, 0.0, 0.0, 1.0]),
//         Vertex::new([ 1.0, -1.0,  1.0], [1.0, 0.0, 1.0, 1.0]),
//         Vertex::new([ 1.0,  1.0,  1.0], [1.0, 1.0, 0.0, 1.0]),
//         Vertex::new([ 1.0,  1.0, -1.0], [1.0, 1.0, 1.0, 1.0]),
//     ];
//
//     let index_data: &[u16] = &[
//         0, 1, 2, 2, 3, 0, //left
//         0, 4, 5, 5, 1, 0, //bottom
//         0, 4, 7, 7, 3, 0, //front
//
//         6, 5, 4, 4, 7, 6, //right
//         6, 2, 3, 3, 7, 6, //top
//         6, 2, 1, 1, 5, 6, //back
//     ];
//
//     let (vbuf, slice) = factory.create_vertex_buffer_with_slice(vertex_data, index_data);
//
//     let data = pipe::Data {
//         vbuf: vbuf,
//         projection_cb: factory.create_constant_buffer(1),
//         out_color: main_color,
//         out_depth: main_depth,
//     };
//
//     let proj_data = ProjectionData {
//         model: {
//             let translation = nalgebra::Vector3::new(0.0, 0.0, 0.0);
//
//             let rotation = nalgebra::Vector3::new(0.0, 0.0, 0.0);
//
//             *nalgebra::Isometry3::new(translation, rotation).to_homogeneous().as_ref()
//         },
//         view: {
//             let eye = nalgebra::Point3::new(10.0, 10.0, 10.0);
//
//             let target = nalgebra::Point3::new(0.0, 0.0, 0.0);
//
//             let up = nalgebra::Vector3::new(0.0, 1.0, 0.0);
//
//             *nalgebra::Isometry3::look_at_rh(&eye, &target, &up).to_homogeneous().as_ref()
//         },
//         proj: {
//             *nalgebra::PerspectiveMatrix3::new(4.0 / 3.0, 75.0, 0.1, 100.0).as_matrix().as_ref()
//         },
//     };
//
//     encoder.update_constant_buffer(&data.projection_cb, &proj_data);
//
//     'main: loop {
//         for event in window.poll_events() {
//             match event {
//                 glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
//                 glutin::Event::Closed => break 'main,
//                 _ => {},
//             }
//         }
//
//         encoder.clear(&data.out_color, [0.0, 0.0, 0.0, 1.0]);
//         encoder.clear_depth(&data.out_depth, 1.0);
//         encoder.draw(&slice, &pso, &data);
//         encoder.flush(&mut device);
//         window.swap_buffers().unwrap();
//         device.cleanup();
//     }
// }


pub struct CompRenderType(usize);

impl specs::Component for CompRenderType {
    type Storage = specs::VecStorage<CompRenderType>;
}

pub struct EncoderChannel<R: gfx_core::Resources, C: gfx_core::draw::CommandBuffer<R>> {
    pub receiver: mpsc::Receiver<gfx::Encoder<R, C>>,
    pub sender: mpsc::Sender<gfx::Encoder<R, C>>,
}

pub struct RenderSystem<R: gfx_core::Resources, C: gfx_core::draw::CommandBuffer<R>> {
    channel: EncoderChannel<R, C>,
    out_color: gfx_core::handle::RenderTargetView<R, ColorFormat>,
    out_depth: gfx_core::handle::DepthStencilView<R, DepthFormat>,
    bundles: Arc<Vec<gfx::Bundle<R, pipe::Data<R>>>>,
}

impl<R: gfx_core::Resources, C: gfx_core::draw::CommandBuffer<R>> RenderSystem<R, C> {
    fn new(channel: EncoderChannel<R, C>, out_color: gfx_core::handle::RenderTargetView<R, ColorFormat>, out_depth: gfx_core::handle::DepthStencilView<R, DepthFormat>) -> RenderSystem<R, C> {
        RenderSystem {
            channel: channel,
            out_color: out_color,
            out_depth: out_depth,
            bundles: Arc::new(Vec::new()),
        }
    }

    fn add_render_type<F: gfx_core::Factory<R>>(&mut self, factory: &mut F, primitive: gfx_core::Primitive, rast: gfx_core::state::Rasterizer, vertices: &[Vertex], indices: &[Index]) -> CompRenderType {
        let pso = factory.create_pipeline_simple (
            include_bytes!("shader/explore_150_v.glsl"),
            include_bytes!("shader/explore_150_f.glsl"),
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
        bundles.push(gfx_core::Bundle::new(slice, pso, data));
        CompRenderType(id)
    }
}

impl<R, C> specs::System<f32> for RenderSystem<R, C> where
R: 'static + gfx_core::Resources,
C: 'static + gfx_core::draw::CommandBuffer<R> + Send,
{
    fn run(&mut self, arg: specs::RunArg, _: f32) {
        use specs::Join;

        let mut encoder = match self.channel.receiver.recv() {
            Ok(r) => r,
            Err(_) => return,
        };

        let (draw, transform, camera) = arg.fetch(|w| {
            (w.read::<CompRenderType>(), w.read::<super::transform::CompTransform>(), w.read::<super::camera::CompCamera>())
        });

        encoder.clear(&self.out_color, [0.0, 0.0, 0.0, 1.0]);
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

        let _ = self.channel.sender.send(encoder);
    }
}
