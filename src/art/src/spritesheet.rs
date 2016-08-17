pub fn make_square_render(texture: ::gfx::handle::ShaderResourceView<::gfx_device_gl::Resources, [f32; 4]>) -> ::graphics::texture::Packet {
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

    ::graphics::texture::Packet::new(vertices, indices, rasterizer, texture)
}
