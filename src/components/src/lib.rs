extern crate gfx;
extern crate gfx_device_gl;
extern crate specs;
extern crate nalgebra;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate graphics;
extern crate utils;
extern crate math;
extern crate art;

pub mod camera;
pub mod clickable;
pub mod dwarf;
pub mod living;
pub mod physical;
pub mod render_data;
pub mod render_type;
// pub mod texture_storage;
pub mod transform;

pub use self::camera::Component as Camera;
pub use self::clickable::Component as Clickable;
pub use self::dwarf::Component as Dwarf;
pub use self::living::Component as Living;
pub use self::physical::Component as Physical;
pub use self::render_data::Component as RenderData;
pub use self::render_type::Component as RenderType;
// pub use self::texture_storage::Component as TextureStorage;
pub use self::transform::Component as Transform;
