use gfx_device_gl::{Factory};
use gfx;

use {Vertex};

pub fn make_square_render(renderer: &mut ::sys::render::System, factory: &mut Factory) -> ::comps::RenderType {
    let vertices = &[
        Vertex::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0, 1.0]),
        Vertex::new([0.0, 1.0, 0.0], [0.0, 1.0, 0.0, 1.0]),
        Vertex::new([1.0, 1.0, 0.0], [1.0, 0.0, 0.0, 1.0]),
        Vertex::new([1.0, 0.0, 0.0], [1.0, 1.0, 1.0, 1.0]),
    ];

    let indices = &[
        // 0, 1, 2, 2, 3, 0, // front culling
        0, 3, 2, 2, 1, 0, // back culling
    ];

    let rasterizer = gfx::state::Rasterizer::new_fill().with_cull_back();

    renderer.add_render_type_complex(
       factory,
       vertices,
       indices,
       rasterizer
   )
}
