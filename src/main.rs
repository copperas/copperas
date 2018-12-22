extern crate gfx;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate winit;
pub mod lib;

static CONFIG_PATH: &'static str = "./configs/config.toml";

// TODO: move all static const here if possible or pipe 'em through lib::config::Config

fn main() {
    lib::core::run(CONFIG_PATH);
}
