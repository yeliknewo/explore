#[macro_use]
extern crate log;
extern crate env_logger;

pub mod fps_counter;

pub type Delta = f32;
pub type Coord = f32;

#[derive(Debug)]
pub enum Error {
    Empty,
    Logged,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            Error::Empty => write!(f, "Error::Empty"),
            Error::Logged => write!(f, "Error::Logged"),
        }
    }
}
