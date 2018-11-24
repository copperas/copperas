extern crate gl;
extern crate glutin;
#[macro_use]
extern crate serde_derive;
extern crate toml;

pub mod lib;

use lib::core::config::Config;
use lib::game::Game;

static CONFIG_PATH: &'static str = "./configs/config.toml";

// TODO: move all static const here if possible or pipe 'em through lib::config::Config

fn main() {
    println!("Init configs");
    let mut config = Config::new(CONFIG_PATH);
    println!("Start new game!");
    let mut game   = Game::new(&mut config);
    game.start();
}
