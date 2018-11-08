extern crate gl;
extern crate glutin;
pub mod lib;

use lib::game::Game;

fn main() {
    println!("Start new game!");
    let mut game = Game::new();
    game.start();
}
