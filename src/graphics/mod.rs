pub mod color;
pub mod texture;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

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
