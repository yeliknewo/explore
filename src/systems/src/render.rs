use gfx::traits::{Factory, FactoryExt};

pub type Channel = (
    ::std::sync::mpsc::Sender<SendEvent>,
    ::std::sync::mpsc::Receiver<RecvEvent>
);

pub enum SendEvent {
    Encoder(::gfx::Encoder<::gfx_device_gl::Resources, ::gfx_device_gl::CommandBuffer>),
}

pub enum RecvEvent {
    Encoder(::gfx::Encoder<::gfx_device_gl::Resources, ::gfx_device_gl::CommandBuffer>),
    GraphicsData(::gfx::handle::RenderTargetView<::gfx_device_gl::Resources, ::graphics::ColorFormat>, ::gfx::handle::DepthStencilView<::gfx_device_gl::Resources, ::graphics::DepthFormat>),
    Exit,
}

pub struct System {
    channel: Channel,
    out_color: ::gfx::handle::RenderTargetView<::gfx_device_gl::Resources, ::graphics::ColorFormat>,
    out_depth: ::gfx::handle::DepthStencilView<::gfx_device_gl::Resources, ::graphics::DepthFormat>,
    color_bundles: ::std::sync::Arc<Vec<::graphics::color::Bundle>>,
    texture_bundles: ::std::sync::Arc<Vec<::graphics::texture::Bundle>>,
    color_shaders: ::graphics::Shaders,
    texture_shaders: ::graphics::Shaders,
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
            color_bundles: ::std::sync::Arc::new(Vec::new()),
            texture_bundles: ::std::sync::Arc::new(Vec::new()),
            color_shaders: ::graphics::color::make_shaders(),
            texture_shaders: ::graphics::texture::make_shaders(),
        }
    }

    pub fn add_render_type_color(&mut self,
        factory: &mut ::gfx_device_gl::Factory,
        color_packet: ::graphics::color::Packet
    ) -> ::comps::RenderType
    {
        self.add_render_type_color_raw(
            factory,
            color_packet.get_vertices(),
            color_packet.get_indices(),
            color_packet.get_rasterizer()
        )
    }

    fn add_render_type_color_raw(&mut self,
        factory: &mut ::gfx_device_gl::Factory,
        vertices: &[::graphics::color::Vertex],
        indices: &[::graphics::color::Index],
        rasterizer: ::gfx::state::Rasterizer
    ) -> ::comps::RenderType
    {
        let shader_set = factory.create_shader_set(self.color_shaders.get_vertex_shader(), self.color_shaders.get_fragment_shader()).unwrap();

        let program = factory.create_program(&shader_set).unwrap();

        let pso = factory.create_pipeline_from_program(&program, ::gfx::Primitive::TriangleList, rasterizer, ::graphics::color::pipe::new()).unwrap();
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(vertices, indices);
        let data = ::graphics::color::pipe::Data {
            vbuf: vbuf,
            projection_cb: factory.create_constant_buffer(1),
            out_color: self.out_color.clone(),
            out_depth: self.out_depth.clone(),
        };
        let id = self.color_bundles.len();
        let mut bundles = ::std::sync::Arc::get_mut(&mut self.color_bundles).unwrap();
        bundles.push(::graphics::color::Bundle::new(slice, pso, data));
        ::comps::RenderType {
            id: id,
            renderer_type: ::graphics::RendererType::Color,
        }
    }

    pub fn add_render_type_texture(&mut self,
        factory: &mut ::gfx_device_gl::Factory,
        mut texture_packet: ::graphics::texture::Packet
    ) -> ::comps::RenderType
    {
        let texture = texture_packet.get_texture();

        self.add_render_type_texture_raw(
            factory,
            texture_packet.get_vertices(),
            texture_packet.get_indices(),
            texture_packet.get_rasterizer(),
            texture
        )
    }

    fn add_render_type_texture_raw(&mut self,
        factory: &mut ::gfx_device_gl::Factory,
        vertices: &[::graphics::texture::Vertex],
        indices: &[::graphics::texture::Index],
        rasterizer: ::gfx::state::Rasterizer,
        texture_view: ::gfx::handle::ShaderResourceView<::gfx_device_gl::Resources, [f32; 4]>
    ) -> ::comps::RenderType
    {
        let shader_set = factory.create_shader_set(self.texture_shaders.get_vertex_shader(), self.texture_shaders.get_fragment_shader()).unwrap();

        let program = factory.create_program(&shader_set).unwrap();

        let pso = factory.create_pipeline_from_program(&program, ::gfx::Primitive::TriangleList, rasterizer, ::graphics::texture::pipe::new()).unwrap();
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(vertices, indices);
        let data = ::graphics::texture::pipe::Data {
            vbuf: vbuf,
            texture: (texture_view, factory.create_sampler_linear()),
            texture_data: factory.create_constant_buffer(1),
            projection_cb: factory.create_constant_buffer(1),
            out_color: self.out_color.clone(),
            out_depth: self.out_depth.clone(),
        };
        let id = self.texture_bundles.len();
        let mut bundles = ::std::sync::Arc::get_mut(&mut self.texture_bundles).unwrap();
        bundles.push(::graphics::texture::Bundle::new(slice, pso, data));
        ::comps::RenderType {
            id: id,
            renderer_type: ::graphics::RendererType::Texture,
        }
    }

    fn render(&mut self, arg: &::specs::RunArg, mut encoder: ::gfx::Encoder<::gfx_device_gl::Resources, ::gfx_device_gl::CommandBuffer>) {
        use specs::Join;

        let (draw, transform, camera, render_data) = arg.fetch(|w| {
            (w.read::<::comps::RenderType>(), w.read::<::comps::Transform>(), w.read::<::comps::Camera>(), w.read::<::comps::RenderData>())
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

        for (d, t, render_data) in (&draw, &transform, &render_data).iter() {
            let projection_data = ::graphics::ProjectionData {
                model: t.get_model(),
                view: view,
                proj: proj,
            };
            match d.renderer_type {
                ::graphics::RendererType::Color => {
                    let b = &self.color_bundles[d.id];
                    encoder.update_constant_buffer(&b.data.projection_cb, &projection_data);
                    b.encode(&mut encoder);
                }
                ::graphics::RendererType::Texture => {
                    let b = &self.texture_bundles[d.id];
                    encoder.update_constant_buffer(&b.data.projection_cb, &projection_data);
                    let texture_data = ::graphics::texture::TextureData {
                        tint: render_data.get_tint(),
                    };
                    encoder.update_constant_buffer(&b.data.texture_data, &texture_data);
                    b.encode(&mut encoder);
                },
            }

        }

        let _ = self.channel.0.send(SendEvent::Encoder(encoder));
    }

    fn set_graphics_data(&mut self, out_color: ::gfx::handle::RenderTargetView<::gfx_device_gl::Resources, ::graphics::ColorFormat>, out_depth: ::gfx::handle::DepthStencilView<::gfx_device_gl::Resources, ::graphics::DepthFormat>) {
        self.out_color = out_color;
        self.out_depth = out_depth;

        for bundle in ::std::sync::Arc::get_mut(&mut self.color_bundles).unwrap() {
            bundle.data.out_color = self.out_color.clone();
            bundle.data.out_depth = self.out_depth.clone();
        }
        for bundle in ::std::sync::Arc::get_mut(&mut self.texture_bundles).unwrap() {
            bundle.data.out_color = self.out_color.clone();
            bundle.data.out_depth = self.out_depth.clone();
        }
    }

    fn exit(&mut self, arg: &::specs::RunArg) {
        //use to save

        arg.fetch(|_| ());
    }

    fn process_event(&mut self, arg: &::specs::RunArg, event: RecvEvent) -> bool {
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

impl ::specs::System<::utils::Delta> for System {
    fn run(&mut self, arg: ::specs::RunArg, _: ::utils::Delta) {
        let mut event = self.channel.1.recv().unwrap();
        while self.process_event(&arg, event) {
            event = self.channel.1.recv().unwrap();
        }
    }
}
