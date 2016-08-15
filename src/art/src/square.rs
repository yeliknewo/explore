use std::io::Read;

pub fn make_square_render(factory: &mut ::gfx_device_gl::Factory) -> ::graphics::texture::Packet {
    let vertices = vec!(
            ::graphics::texture::Vertex::new([0.0, 0.0, 0.0], [1.0, 1.0]),
            ::graphics::texture::Vertex::new([0.0, 1.0, 0.0], [1.0, 0.0]),
            ::graphics::texture::Vertex::new([1.0, 1.0, 0.0], [0.0, 0.0]),
            ::graphics::texture::Vertex::new([1.0, 0.0, 0.0], [0.0, 1.0]),
        );

    let indices = vec!(
            0, 3, 2, 2, 1, 0,
        );

    let mut texture_path = ::find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    texture_path.push("square.jpg");

    let texture_file = ::std::fs::File::open(texture_path).unwrap();

    let mut texture_reader = ::std::io::BufReader::new(texture_file);

    let mut texture_buffer = vec!();

    texture_reader.read_to_end(&mut texture_buffer).unwrap();

    let texture = ::graphics::texture::load_texture(factory, &texture_buffer).unwrap();

    let rasterizer = ::gfx::state::Rasterizer::new_fill().with_cull_back();

    ::graphics::texture::Packet::new(vertices, indices, texture, rasterizer)
}
