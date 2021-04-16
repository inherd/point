#[macro_use]
extern crate log;

pub mod print_ui;

pub fn main() {
    env_logger::init();
    info!("starting up");
}
