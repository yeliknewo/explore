use std::io::Read;

pub fn make_square_render(factory: &mut ::gfx_device_gl::Factory) -> Result<::graphics::texture::Packet, ::utils::Error> {
    let vertices = vec!(
            ::graphics::texture::Vertex::new([0.0, 0.0, 0.0], [1.0, 1.0]),
            ::graphics::texture::Vertex::new([0.0, 1.0, 0.0], [1.0, 0.0]),
            ::graphics::texture::Vertex::new([1.0, 1.0, 0.0], [0.0, 0.0]),
            ::graphics::texture::Vertex::new([1.0, 0.0, 0.0], [0.0, 1.0]),
        );

    let indices = vec!(
            0, 3, 2, 2, 1, 0,
        );

    let mut texture_path = match ::find_folder::Search::ParentsThenKids(3, 3).for_folder("assets") {
        Ok(texture_path) => texture_path,
        Err(err) => {
            error!("Texture path error: {}", err);
            return Err(::utils::Error::Logged);
        },
    };
    texture_path.push("square.jpg");

    let texture_file = match ::std::fs::File::open(texture_path) {
        Ok(texture_file) => texture_file,
        Err(err) => {
            error!("Texture file error: {}", err);
            return Err(::utils::Error::Logged);
        },
    };

    let mut texture_reader = ::std::io::BufReader::new(texture_file);

    let mut texture_buffer = vec!();

    match texture_reader.read_to_end(&mut texture_buffer) {
        Ok(_) => (),
        Err(err) => {
            error!("Texture reader read to end error: {}", err);
            return Err(::utils::Error::Logged);
        },
    }

    let texture = try!(::graphics::texture::load_texture(factory, &texture_buffer));

    let rasterizer = ::gfx::state::Rasterizer::new_fill().with_cull_back();

    Ok(::graphics::texture::Packet::new(vertices, indices, texture, rasterizer))
}
