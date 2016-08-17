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

pub const GRASS_MID: usize = 0;
pub const GRASS_CENTER: usize = 1;

pub const P1_WALK01: usize = 2;
pub const P1_WALK02: usize = 3;
pub const P1_WALK03: usize = 4;
pub const P1_WALK04: usize = 5;
pub const P1_WALK05: usize = 6;
pub const P1_WALK06: usize = 7;
pub const P1_WALK07: usize = 8;
pub const P1_WALK08: usize = 9;
pub const P1_WALK09: usize = 10;
pub const P1_WALK10: usize = 11;
pub const P1_WALK11: usize = 12;

pub const P1_FRONT: usize = 13;
pub const P1_STAND: usize = 14;
pub const P1_DUCK: usize = 15;
pub const P1_HURT: usize = 16;
pub const P1_JUMP: usize = 17;

pub fn make_texture_storage_vec(factory: &mut ::gfx_device_gl::Factory) -> Vec<::gfx::handle::ShaderResourceView<::gfx_device_gl::Resources, [f32; 4]>>{
    let mut vec = vec!();

    let assets_folder = match ::find_folder::Search::ParentsThenKids(3, 3).for_folder("assets") {
        Ok(path) => path,
        Err(err) => {
            error!("error finding assets folder: {}", err);
            return vec;
        }
    };

    let paths = make_paths();

    for p in &paths {
        load_texture(factory, &mut vec, &assets_folder, p);
    }

    vec
}

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

fn make_paths() -> Vec<&'static str> {
    let mut vec = vec!();

    vec[GRASS_MID] = "Tiles/grassMid.png";
    vec[GRASS_CENTER] = "Tiles/grassCenter.png";

    vec[P1_WALK01] = "Player/p1_walk/PNG/p1_walk01.png";
    vec[P1_WALK02] = "Player/p1_walk/PNG/p1_walk02.png";
    vec[P1_WALK03] = "Player/p1_walk/PNG/p1_walk03.png";
    vec[P1_WALK04] = "Player/p1_walk/PNG/p1_walk04.png";
    vec[P1_WALK05] = "Player/p1_walk/PNG/p1_walk05.png";
    vec[P1_WALK06] = "Player/p1_walk/PNG/p1_walk06.png";
    vec[P1_WALK07] = "Player/p1_walk/PNG/p1_walk07.png";
    vec[P1_WALK08] = "Player/p1_walk/PNG/p1_walk08.png";
    vec[P1_WALK09] = "Player/p1_walk/PNG/p1_walk09.png";
    vec[P1_WALK10] = "Player/p1_walk/PNG/p1_walk010.png";
    vec[P1_WALK11] = "Player/p1_walk/PNG/p1_walk011.png";

    vec[P1_FRONT] = "Player/p1_front.png";
    vec[P1_STAND] = "Player/p1_stand.png";
    vec[P1_DUCK] = "Player/p1_duck.png";
    vec[P1_HURT] = "Player/p1_hurt.png";
    vec[P1_JUMP] = "Player/p1_jump.png";

    vec
}
