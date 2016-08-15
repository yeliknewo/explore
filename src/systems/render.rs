use std::sync::{mpsc, Arc};
use gfx;
use gfx_core;
use gfx::traits::{Factory, FactoryExt};
use gfx_device_gl;
use gfx_window_glutin;
use glutin;
use specs;

trait BundleTrait {
    fn encode(&self, encoder: &mut gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>);
}

trait DataMaker<D: gfx::pso::PipelineData<gfx_device_gl::Resources>, V> {
    fn make_data(&self, vbuf: gfx::VertexBuffer<V>) -> D;
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
    GraphicsData(gfx::handle::RenderTargetView<gfx_device_gl::Resources, ::graphics::ColorFormat>, gfx::handle::DepthStencilView<gfx_device_gl::Resources, ::graphics::DepthFormat>),
    Exit,
}

pub struct System {
    channel: Channel,
    out_color: gfx::handle::RenderTargetView<gfx_device_gl::Resources, ::graphics::ColorFormat>,
    out_depth: gfx::handle::DepthStencilView<gfx_device_gl::Resources, ::graphics::DepthFormat>,
    bundles: Arc<Vec<Box<BundleTrait>>>,
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

    // pub fn add_render_type(&mut self,
    //     factory: &mut gfx_device_gl::Factory,
    //     vertices: &[Vertex],
    //     indices: &[Index]
    // ) -> ::comps::RenderType
    // {
    //     let pso = factory.create_pipeline_simple (
    //         VERTEX_SHADER,
    //         FRAGMENT_SHADER,
    //         pipe::new()
    //     ).unwrap();
    //     let (vbuf, slice) = factory.create_vertex_buffer_with_slice(vertices, indices);
    //     let data = pipe::Data {
    //         vbuf: vbuf,
    //         projection_cb: factory.create_constant_buffer(1),
    //         out_color: self.out_color.clone(),
    //         out_depth: self.out_depth.clone(),
    //     };
    //     let id = self.bundles.len();
    //     let mut bundles = Arc::get_mut(&mut self.bundles).unwrap();
    //     bundles.push(Bundle::new(slice, pso, data));
    //     ::comps::RenderType(id)
    // }



    ///EXPERIMENTAL

    pub fn add_render_type_generic<P: gfx::pso::PipelineInit, V, I, B: BundleTrait>(&mut self,
        factory: &mut gfx_device_gl::Factory,
        vertices: Box<&[V]>,
        indices: Box<&[I]>,
        vertex_shader: &[u8],
        fragment_shader: &[u8],
        primitive: gfx::Primitive,
        rasterizer: gfx::state::Rasterizer,
        data_maker: Box<DataMaker<gfx::pso::PipelineData<gfx_device_gl::Resources>, V>>
    ) -> ::comps::RenderType
    {
        let shader_set = factory.create_shader_set(vertex_shader, fragment_shader).unwrap();

        let program = factory.create_program(&shader_set).unwrap();

        let pso = factory.create_pipeline_from_program(&program, primitive, rasterizer, P::new()).unwrap();

        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(vertices, indices);

        let data = data_maker.make_data(vbuf);

        let id = self.bundles.len();
        let mut bundles = Arc::get_mut(&mut self.bundles).unwrap();
        bundles.push(DataMaker::new_bundle(slice, pso, data));
        ::comps::RenderType(id)
    }

    // pub fn add_render_type_complex(&mut self,
    //     factory: &mut gfx_device_gl::Factory,
    //     vertices: &[color::Vertex],
    //     indices: &[color::Index],
    //     rasterizer: gfx::state::Rasterizer
    // ) -> ::comps::RenderType
    // {
    //     let shader_set = factory.create_shader_set(VERTEX_SHADER, FRAGMENT_SHADER).unwrap();
    //
    //     let program = factory.create_program(&shader_set).unwrap();
    //
    //     let pso = factory.create_pipeline_from_program(&program, gfx::Primitive::TriangleList, rasterizer, pipe::new()).unwrap();
    //     let (vbuf, slice) = factory.create_vertex_buffer_with_slice(vertices, indices);
    //     let data = pipe::Data {
    //         vbuf: vbuf,
    //         projection_cb: factory.create_constant_buffer(1),
    //         out_color: self.out_color.clone(),
    //         out_depth: self.out_depth.clone(),
    //     };
    //     let id =self.bundles.len();
    //     let mut bundles = Arc::get_mut(&mut self.bundles).unwrap();
    //     bundles.push(Bundle::new(slice, pso, data));
    //     ::comps::RenderType(id)
    // }

    // pub fn add_render_type_texture(&mut self,
    //     factory: &mut gfx_device_gl::Factory,
    //     vertices: &[texture::Vertex],
    //     indices: &[texture::Index],
    //     rasterizer: gfx::state::Rasterizer
    // ) -> ::comps::RenderType
    // {
    //
    // }

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
            // let projection_data = ProjectionData {
            //     model: t.get_model(),
            //     view: view,
            //     proj: proj,
            // };
            let b = &self.bundles[d.0];
            // encoder.update_constant_buffer(&b.data.projection_cb, &projection_data);
            b.encode(&mut encoder);
        }

        let _ = self.channel.0.send(SendEvent::Encoder(encoder));
    }

    fn set_graphics_data(&mut self, out_color: gfx::handle::RenderTargetView<gfx_device_gl::Resources, ::graphics::ColorFormat>, out_depth: gfx::handle::DepthStencilView<gfx_device_gl::Resources, ::graphics::DepthFormat>) {
        self.out_color = out_color;
        self.out_depth = out_depth;

        for bundle in Arc::get_mut(&mut self.bundles).unwrap() {
            bundle.update_data(self.out_color.clone(), self.out_depth.clone());
        }
    }

    fn exit(&mut self, arg: &specs::RunArg) {
        //use to save

        arg.fetch(|_| ());
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
            RecvEvent::Exit => {
                self.exit(arg);
                false
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
