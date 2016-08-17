extern crate nalgebra;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate core;
extern crate art;
extern crate components as comps;
extern crate math;
extern crate utils;

fn main() {
    env_logger::init().unwrap_or_else(
        |err|
            panic!("unable to initiate env logger: {}", err)
    );

    match core::start() {
        Ok(()) => info!("game exiting"),
        Err(err) => error!("core start error: {}", err),
    }
}
