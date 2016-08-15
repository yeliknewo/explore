mod square;

pub use self::square::make_square_render;

// pub fn make_cube_render(renderer: &mut RenderSystem, factory: &mut Factory) -> CompRenderType {
//     let vertices = &[
//         Vertex::new([-1.0, -1.0, -1.0], [0.0, 0.0, 0.0, 1.0]),
//         Vertex::new([-1.0, -1.0,  1.0], [0.0, 0.0, 1.0, 1.0]),
//         Vertex::new([-1.0,  1.0,  1.0], [0.0, 1.0, 0.0, 1.0]),
//         Vertex::new([-1.0,  1.0, -1.0], [0.0, 1.0, 1.0, 1.0]),
//         Vertex::new([ 1.0, -1.0, -1.0], [1.0, 0.0, 0.0, 1.0]),
//         Vertex::new([ 1.0, -1.0,  1.0], [1.0, 0.0, 1.0, 1.0]),
//         Vertex::new([ 1.0,  1.0,  1.0], [1.0, 1.0, 0.0, 1.0]),
//         Vertex::new([ 1.0,  1.0, -1.0], [1.0, 1.0, 1.0, 1.0]),
//     ];
//
//     let indices = &[
//         0, 1, 2, 2, 3, 0,
//         0, 4, 5, 5, 1, 0,
//         0, 4, 7, 7, 3, 0,
//
//         6, 5, 4, 4, 7, 6,
//         6, 2, 3, 3, 7, 6,
//         6, 2, 1, 1, 5, 6,
//     ];
//
//     renderer.add_render_type(
//        factory,
//        vertices,
//        indices
//    )
// }
