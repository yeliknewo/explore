use gfx::traits::{Factory, FactoryExt};

pub type Channel = (
    ::std::sync::mpsc::Sender<SendEvent>,
    ::std::sync::mpsc::Receiver<RecvEvent>
);

pub enum SendEvent {
    Encoder(::gfx::Encoder<::gfx_device_gl::Resources, ::gfx_device_gl::CommandBuffer>),
    Error(::utils::Error),
    Exited,
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
    spritesheet_bundles: ::std::sync::Arc<Vec<::graphics::spritesheet::Bundle>>,
    color_shaders: ::graphics::Shaders,
    texture_shaders: ::graphics::Shaders,
    spritesheet_shaders: ::graphics::Shaders,
    exited: bool,
}

impl System {
    pub fn new(
        channel: Channel
    ) -> Result<System, ::utils::Error>
    {
        let (out_color, out_depth) = match channel.1.recv() {
            Ok(event) => match event {
                RecvEvent::GraphicsData(out_color, out_depth) => (out_color, out_depth),
                _ => panic!("render system received non graphics data first from channel"),
            },
            Err(err) => {
                error!("new channel 1 rect error: {}", err);
                return Err(::utils::Error::Logged);
            }
        } ;

        Ok(System {
            channel: channel,
            out_color: out_color,
            out_depth: out_depth,
            color_bundles: ::std::sync::Arc::new(Vec::new()),
            texture_bundles: ::std::sync::Arc::new(Vec::new()),
            spritesheet_bundles: ::std::sync::Arc::new(Vec::new()),
            color_shaders: try!(::graphics::color::make_shaders()),
            texture_shaders: try!(::graphics::texture::make_shaders()),
            spritesheet_shaders: try!(::graphics::spritesheet::make_shaders()),
            exited: false,
        })
    }

    pub fn add_render_type_color(&mut self,
        factory: &mut ::gfx_device_gl::Factory,
        color_packet: ::graphics::color::Packet
    ) -> Result<::comps::RenderType, ::utils::Error> {
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
    ) -> Result<::comps::RenderType, ::utils::Error> {
        let shader_set = match factory.create_shader_set(self.color_shaders.get_vertex_shader(), self.color_shaders.get_fragment_shader()) {
            Ok(shaders) => shaders,
            Err(err) => {
                error!("add render type color raw shader set error: {}", err);
                return Err(::utils::Error::Logged);
            }
        };

        let program = match factory.create_program(&shader_set) {
            Ok(program) => program,
            Err(err) => {
                error!("add render type color raw create program error: {}", err);
                return Err(::utils::Error::Logged);
            }
        };

        let pso = match factory.create_pipeline_from_program(&program, ::gfx::Primitive::TriangleList, rasterizer, ::graphics::color::pipe::new()) {
            Ok(pipeline) => pipeline,
            Err(err) => {
                error!("add render type color raw create pipeline error: {}", err);
                return Err(::utils::Error::Logged);
            }
        };
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(vertices, indices);
        let data = ::graphics::color::pipe::Data {
            vbuf: vbuf,
            projection_cb: factory.create_constant_buffer(1),
            out_color: self.out_color.clone(),
            out_depth: self.out_depth.clone(),
        };
        let id = self.color_bundles.len();
        let mut bundles = match ::std::sync::Arc::get_mut(&mut self.color_bundles) {
            Some(bundles) => bundles,
            None => {
                error!("add render type color raw bundles get mut was none");
                return Err(::utils::Error::Logged);
            }
        };
        bundles.push(::graphics::color::Bundle::new(slice, pso, data));
        Ok(::comps::RenderType {
            id: id,
            renderer_type: ::graphics::RendererType::Color,
        })
    }

    pub fn add_render_type_texture(&mut self,
        factory: &mut ::gfx_device_gl::Factory,
        mut texture_packet: ::graphics::texture::Packet
    ) -> Result<::comps::RenderType, ::utils::Error> {
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
    ) -> Result<::comps::RenderType, ::utils::Error> {
        let shader_set = match factory.create_shader_set(self.texture_shaders.get_vertex_shader(), self.texture_shaders.get_fragment_shader()) {
            Ok(shaders) => shaders,
            Err(err) => {
                error!("add render type texture raw create shader set error: {}", err);
                return Err(::utils::Error::Logged);
            }
        };

        let program = match factory.create_program(&shader_set) {
            Ok(program) => program,
            Err(err) => {
                error!("add render type texture raw create program error: {}", err);
                return Err(::utils::Error::Logged);
            }
        };

        let pso = match factory.create_pipeline_from_program(
            &program,
            ::gfx::Primitive::TriangleList,
            rasterizer,
            ::graphics::texture::pipe::new()
        ) {
            Ok(pso) => pso,
            Err(err) => {
                error!("add render type texture raw create pipeline error: {}", err);
                return Err(::utils::Error::Logged);
            }
        };
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
        let mut bundles = match ::std::sync::Arc::get_mut(&mut self.texture_bundles) {
            Some(bundles) => bundles,
            None => {
                error!("add render type texture raw get mut bundles was none");
                return Err(::utils::Error::Logged);
            }
        };
        bundles.push(::graphics::texture::Bundle::new(slice, pso, data));
        Ok(::comps::RenderType {
            id: id,
            renderer_type: ::graphics::RendererType::Texture,
        })
    }

    pub fn add_render_type_spritesheet(&mut self,
        factory: &mut ::gfx_device_gl::Factory,
        packet: &::graphics::spritesheet::Packet,
        texture: ::gfx::handle::ShaderResourceView<::gfx_device_gl::Resources, [f32; 4]>
    ) -> Result<::comps::RenderType, ::utils::Error> {

        self.add_render_type_spritesheet_raw(factory, packet.get_vertices(), packet.get_indices(), packet.get_rasterizer(), texture)
    }

    fn add_render_type_spritesheet_raw(&mut self,
        factory: &mut ::gfx_device_gl::Factory,
        vertices: &[::graphics::spritesheet::Vertex],
        indices: &[::graphics::spritesheet::Index],
        rasterizer: ::gfx::state::Rasterizer,
        spritesheet: ::gfx::handle::ShaderResourceView<::gfx_device_gl::Resources, [f32; 4]>
    ) -> Result<::comps::RenderType, ::utils::Error> {
        let shader_set = match factory.create_shader_set(self.spritesheet_shaders.get_vertex_shader(), self.spritesheet_shaders.get_fragment_shader()) {
            Ok(shaders) => shaders,
            Err(err) => {
                error!("add render type spritesheet raw create shader set error: {}", err);
                return Err(::utils::Error::Logged);
            }
        };

        let program  = match factory.create_program(&shader_set) {
            Ok(program) => program,
            Err(err) => {
                error!("add render type spritesheet raw create program error: {}", err);
                return Err(::utils::Error::Logged);
            }
        };

        let pso = match factory.create_pipeline_from_program(
            &program,
            ::gfx::Primitive::TriangleList,
            rasterizer,
            ::graphics::spritesheet::pipe::new()
        ) {
            Ok(pso) => pso,
            Err(err) => {
                error!("add render type spritesheet raw create pipeline error: {}", err);
                return Err(::utils::Error::Logged);
            }
        };

        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(vertices, indices);
        let data = ::graphics::spritesheet::pipe::Data {
            vbuf: vbuf,
            spritesheet: (spritesheet, factory.create_sampler_linear()),
            texture_data: factory.create_constant_buffer(1),
            projection_cb: factory.create_constant_buffer(1),
            out_color: self.out_color.clone(),
            out_depth: self.out_depth.clone(),
        };

        let id = self.spritesheet_bundles.len();
        let mut bundles = match ::std::sync::Arc::get_mut(&mut self.spritesheet_bundles) {
            Some(bundles) => bundles,
            None => {
                error!("add render type spritesheet raw get mut bundles was none");
                return Err(::utils::Error::Logged);
            }
        };
        bundles.push(::graphics::spritesheet::Bundle::new(slice, pso, data));
        Ok(::comps::RenderType {
            id: id,
            renderer_type: ::graphics::RendererType::Spritesheet,
        })
    }

    fn render(&mut self, arg: &::specs::RunArg, mut encoder: ::gfx::Encoder<::gfx_device_gl::Resources, ::gfx_device_gl::CommandBuffer>) -> Result<(), ::utils::Error> {
        use specs::Join;

        let (draw, transform, mut camera, mut render_data) = arg.fetch(|w| {
            (w.read::<::comps::RenderType>(), w.read::<::comps::Transform>(), w.write::<::comps::Camera>(), w.write::<::comps::RenderData>())
        });

        encoder.clear(&self.out_color, [0.0, 0.0, 0.0, 1.0]);
        encoder.clear_depth(&self.out_depth, 1.0);

        let (view, proj, dirty_cam) = {
            let mut camera = {
                let mut camera_opt = None;

                for c in (&mut camera).iter() {
                    camera_opt = Some(c);
                }

                match camera_opt {
                    Some(camera) => camera,
                    None => {
                        error!("render camera opt was none");
                        return Err(::utils::Error::Logged);
                    }
                }
            };

            (camera.get_view(), camera.get_proj(), camera.take_dirty())
        };

        for (d, t, mut rd) in (&draw, &transform, &mut render_data).iter() {
            match d.renderer_type {
                ::graphics::RendererType::Color => {
                    let b = &self.color_bundles[d.id];

                    if dirty_cam {
                        let projection_data = ::graphics::ProjectionData {
                            model: t.get_model(),
                            view: view,
                            proj: proj,
                        };
                        encoder.update_constant_buffer(&b.data.projection_cb, &projection_data);
                    }

                    b.encode(&mut encoder);
                }
                ::graphics::RendererType::Texture => {
                    let b = &self.texture_bundles[d.id];

                    if dirty_cam {
                        let projection_data = ::graphics::ProjectionData {
                            model: t.get_model(),
                            view: view,
                            proj: proj,
                        };
                        encoder.update_constant_buffer(&b.data.projection_cb, &projection_data);
                    }

                    if rd.take_dirty() {
                        let texture_data = ::graphics::texture::TextureData {
                            tint: rd.get_tint(),
                        };
                        encoder.update_constant_buffer(&b.data.texture_data, &texture_data);
                    }

                    b.encode(&mut encoder);
                },
                ::graphics::RendererType::Spritesheet => {
                    let b = &self.spritesheet_bundles[d.id];

                    if dirty_cam {
                        let projection_data = ::graphics::ProjectionData {
                            model: t.get_model(),
                            view: view,
                            proj: proj,
                        };
                        encoder.update_constant_buffer(&b.data.projection_cb, &projection_data);
                    }

                    if rd.take_dirty() {
                        let texture_data = ::graphics::spritesheet::TextureData {
                            tint: rd.get_tint(),
                            spritesheet_rect: rd.get_spritesheet_rect(),
                            spritesheet_size: rd.get_spritesheet_size(),
                        };
                        encoder.update_constant_buffer(&b.data.texture_data, &texture_data);
                    }

                    b.encode(&mut encoder);
                }
            }
        }

        match self.channel.0.send(SendEvent::Encoder(encoder)) {
            Ok(()) => (),
            Err(err) => {
                error!("render channel 0 send error: {}", err);
                return Err(::utils::Error::Logged);
            }
        }

        Ok(())
    }

    fn set_graphics_data(&mut self, out_color: ::gfx::handle::RenderTargetView<::gfx_device_gl::Resources, ::graphics::ColorFormat>, out_depth: ::gfx::handle::DepthStencilView<::gfx_device_gl::Resources, ::graphics::DepthFormat>) -> Result<(), ::utils::Error> {
        self.out_color = out_color;
        self.out_depth = out_depth;

        for bundle in match ::std::sync::Arc::get_mut(&mut self.color_bundles) {
            Some(bundle) => bundle,
            None => {
                error!("set graphics data get mut color bundles was none");
                return Err(::utils::Error::Logged);
            }
        } {
            bundle.data.out_color = self.out_color.clone();
            bundle.data.out_depth = self.out_depth.clone();
        }
        for bundle in match ::std::sync::Arc::get_mut(&mut self.texture_bundles) {
            Some(bundle) => bundle,
            None => {
                error!("set graphics data get mut texture bundles was none");
                return Err(::utils::Error::Logged);
            }
        }  {
            bundle.data.out_color = self.out_color.clone();
            bundle.data.out_depth = self.out_depth.clone();
        }

        Ok(())
    }

    fn exit(&mut self, arg: &::specs::RunArg) {
        //use to save

        arg.fetch(|_| ());
    }

    fn process_event(&mut self, arg: &::specs::RunArg, event: RecvEvent) -> Result<bool, ::utils::Error> {
        match event {
            RecvEvent::Encoder(encoder) => {
                try!(self.render(arg, encoder));
                Ok(false)
            },
            RecvEvent::GraphicsData(out_color, out_depth) => {
                try!(self.set_graphics_data(out_color, out_depth));
                Ok(true)
            },
            RecvEvent::Exit => {
                self.exit(arg);
                match self.channel.0.send(SendEvent::Exited) {
                    Ok(()) => (),
                    Err(err) => error!("process event exit send error: {}", err),
                }
                self.exited = true;
                Ok(false)
            },
        }
    }
}

impl ::specs::System<::utils::Delta> for System {
    fn run(&mut self, arg: ::specs::RunArg, _: ::utils::Delta) {
        if self.exited {
            arg.fetch(|_| ());
            return;
        }

        let mut event = match self.channel.1.recv() {
            Ok(event) => event,
            Err(err) => {
                error!("run channel 1 recv error: {}", err);
                self.channel.0.send(SendEvent::Error(::utils::Error::Logged)).unwrap(); //no solution
                return;
            },
        };
        while match self.process_event(&arg, event) {
            Ok(b) => b,
            Err(err) => {
                self.channel.0.send(SendEvent::Error(err)).unwrap(); //no solution
                return;
            },
        } {
            event = match self.channel.1.recv() {
                Ok(event) => event,
                Err(err) => {
                    error!("run channel 1 recv error: {}", err);
                    self.channel.0.send(SendEvent::Error(::utils::Error::Logged)).unwrap(); //no solution
                    return;
                },
            };
        }
    }
}
