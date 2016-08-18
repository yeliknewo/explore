pub fn make_square_render() -> ::graphics::spritesheet::Packet {
    let vertices = vec!(
        ::graphics::spritesheet::Vertex::new([0.0, 0.0, 0.0], [1.0, 1.0]),
        ::graphics::spritesheet::Vertex::new([0.0, 1.0, 0.0], [1.0, 0.0]),
        ::graphics::spritesheet::Vertex::new([1.0, 1.0, 0.0], [0.0, 0.0]),
        ::graphics::spritesheet::Vertex::new([1.0, 0.0, 0.0], [0.0, 1.0]),
    );

    let indices = vec!(
        0, 3, 2, 2, 1, 0,
    );

    let rasterizer = ::gfx::state::Rasterizer::new_fill();

    ::graphics::spritesheet::Packet::new(vertices, indices, rasterizer)
}

pub mod p1 {
    pub const 
}
