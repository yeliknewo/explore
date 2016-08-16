#[macro_use]
extern crate log;
extern crate env_logger;

pub type Delta = f32;
pub type Coord = f32;

#[derive(Debug)]
pub enum Error {
    Empty,
    Logged,
}
