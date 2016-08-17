#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate glutin;
extern crate gfx_window_glutin;
extern crate image;
extern crate find_folder;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate utils;

use std::io::Read;

pub mod color;
pub mod texture;
pub mod spritesheet;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

#[derive(Debug)]
pub struct Shaders {
    vertex: Vec<u8>,
    fragment: Vec<u8>,
}

impl Shaders {
    pub fn new(vertex_name: &'static str, fragment_name: &'static str) -> Result<Shaders, ::utils::Error> {
        let shaders_path = match ::find_folder::Search::ParentsThenKids(3, 3).for_folder("shader") {
            Ok(shaders_path) => shaders_path,
            Err(err) => {
                error!("find folder shader error: {}", err);
                return Err(::utils::Error::Logged);
            }
        };

        let mut vertex_path = shaders_path.clone();
        let mut fragment_path = shaders_path.clone();

        vertex_path.push(vertex_name);
        fragment_path.push(fragment_name);

        let vertex_file = match ::std::fs::File::open(vertex_path) {
            Ok(file) => file,
            Err(err) => {
                error!("vertex file open error: {}", err);
                return Err(::utils::Error::Logged);
            }
        };
        let fragment_file = match ::std::fs::File::open(fragment_path) {
            Ok(file) => file,
            Err(err) => {
                error!("fragment file open error: {}", err);
                return Err(::utils::Error::Logged);
            }
        };

        let mut vertex_reader = ::std::io::BufReader::new(vertex_file);
        let mut fragment_reader = ::std::io::BufReader::new(fragment_file);

        let mut vertex_buffer = vec!();
        let mut fragment_buffer = vec!();

        match vertex_reader.read_to_end(&mut vertex_buffer) {
            Ok(_) => (),
            Err(err) => {
                error!("vertex reader read to end error: {}", err);
                return Err(::utils::Error::Logged);
            }
        };
        match fragment_reader.read_to_end(&mut fragment_buffer) {
            Ok(_) => (),
            Err(err) => {
                error!("fragment reader read to end error: {}", err);
                return Err(::utils::Error::Logged);
            }
        }

        Ok(Shaders {
            vertex: vertex_buffer,
            fragment: fragment_buffer,
        })
    }

    pub fn get_vertex_shader(&self) -> &[u8] {
        self.vertex.as_slice()
    }

    pub fn get_fragment_shader(&self) -> &[u8] {
        self.fragment.as_slice()
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq)]
pub enum RendererType {
    Color,
    Texture,
}

pub fn build_graphics(width: u32, height: u32) -> (
    (gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>, gfx::handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat>),
    gfx_device_gl::Factory,
    gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
    glutin::Window,
    gfx_device_gl::Device
) {
    let builder = glutin::WindowBuilder::new()
        .with_title("Explore")
        .with_dimensions(width, height)
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

gfx_constant_struct!(
    ProjectionData {
        model: [[f32; 4]; 4] = "u_Model",
        view: [[f32; 4]; 4] = "u_View",
        proj: [[f32; 4]; 4] = "u_Proj",
    }
);
