use std::io::Read;

fn make_square_render() -> ::graphics::texture::Packet {
    let vertices = vec!(
        ::graphics::texture::Vertex::new([0.0, 0.0, 0.0], [1.0, 1.0]),
        ::graphics::texture::Vertex::new([0.0, 1.0, 0.0], [1.0, 0.0]),
        ::graphics::texture::Vertex::new([1.0, 1.0, 0.0], [0.0, 0.0]),
        ::graphics::texture::Vertex::new([1.0, 0.0, 0.0], [0.0, 1.0]),
    );

    let indices = vec!(
        0, 3, 2, 2, 1, 0,
    );

    let rasterizer = ::gfx::state::Rasterizer::new_fill().with_cull_back();

    ::graphics::texture::Packet::new_option(vertices, indices, None, rasterizer)
}

fn load_texture(factory: &mut ::gfx_device_gl::Factory, string: &'static str) -> Result<::gfx::handle::ShaderResourceView<::gfx_device_gl::Resources, [f32; 4]>, ::utils::Error> {
    let mut texture_path = match ::find_folder::Search::ParentsThenKids(3, 3).for_folder("assets") {
        Ok(texture_path) => texture_path,
        Err(err) => {
            error!("Texture path error: {}", err);
            return Err(::utils::Error::Logged);
        },
    };
    texture_path.push(string);

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

    ::graphics::texture::load_texture(factory, &texture_buffer)
}

pub fn make_grass_render(factory: &mut ::gfx_device_gl::Factory) -> Result<::graphics::texture::Packet, ::utils::Error> {
    let mut packet = make_square_render();

    packet.set_texture(try!(load_texture(factory, "Tiles/grassMid.png")));

    Ok(packet)
}

pub fn make_grass_center_render(factory: &mut ::gfx_device_gl::Factory) -> Result<::graphics::texture::Packet, ::utils::Error> {
    let mut packet = make_square_render();

    packet.set_texture(try!(load_texture(factory, "Tiles/grassCenter.png")));

    Ok(packet)
}
