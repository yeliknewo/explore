use gfx_device_gl::{Factory};

use {CompRenderType, RenderSystem};

pub fn make_player_render(renderer: &mut RenderSystem, factory: &mut Factory) -> CompRenderType {
    let vertices = &[

    ];

    let indices = &[

    ];

    renderer.add_render_type(
       factory,
       vertices,
       indices
   )
}
