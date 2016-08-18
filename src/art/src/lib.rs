extern crate gfx;
extern crate gfx_device_gl;
extern crate find_folder;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate graphics;
extern crate utils;

pub mod texture;
pub mod spritesheet;

fn load_texture<P>(factory: &mut ::gfx_device_gl::Factory, vec: &mut Vec<::gfx::handle::ShaderResourceView<::gfx_device_gl::Resources, [f32; 4]>>, folder: &std::path::PathBuf, file: P) where P: AsRef<::std::path::Path> + std::fmt::Display {
    if let Ok(texture) = ::graphics::texture::load_texture(factory, folder.join(&file)) {
        vec.push(texture);
    } else if let Ok(texture) = ::graphics::texture::load_texture(factory, folder.join("error.png")) {
        error!("failed to load texture: {}", file);
        vec.push(texture);
    } else {
        error!("failed to load error texture");
    }
}
