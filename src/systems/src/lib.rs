extern crate gfx;
extern crate gfx_device_gl;
extern crate glutin;
extern crate specs;
extern crate nalgebra;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate graphics;
extern crate utils;
extern crate components as comps;
extern crate math;

pub mod control;
pub mod dwarf;
pub mod living;
pub mod physical;
pub mod render;

pub use self::control::System as Control;
pub use self::dwarf::System as Dwarf;
pub use self::living::System as Living;
pub use self::physical::System as Physical;
pub use self::render::System as Render;
