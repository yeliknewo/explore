use gfx_device_gl::{Factory};
use gfx;

use graphics::texture::{Vertex};

pub fn make_square_render(renderer: &mut ::sys::render::System, factory: &mut Factory) -> ::comps::RenderType {
    let vertices = &[
        Vertex::new([0.0, 0.0, 0.0], [1.0, 1.0]),
        Vertex::new([0.0, 1.0, 0.0], [1.0, 0.0]),
        Vertex::new([1.0, 1.0, 0.0], [0.0, 0.0]),
        Vertex::new([1.0, 0.0, 0.0], [0.0, 1.0]),
    ];

    let indices = &[
        // 0, 1, 2, 2, 3, 0, // front culling
        0, 3, 2, 2, 1, 0, // back culling
    ];

    let texture = ::graphics::texture::load_texture(factory, include_bytes!("../../assets/square.jpg")).unwrap();

    let rasterizer = gfx::state::Rasterizer::new_fill().with_cull_back();

    renderer.add_render_type_texture(
       factory,
       vertices,
       indices,
       texture,
       rasterizer
   )
}
