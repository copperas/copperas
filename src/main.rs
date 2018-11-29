extern crate gfx;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate winit;
pub mod lib;

use lib::core::Core;
// use lib::game::Game;

static CONFIG_PATH: &'static str = "./configs/config.toml";

// TODO: move all static const here if possible or pipe 'em through lib::config::Config

fn main() {
    Core::run(CONFIG_PATH);
}
