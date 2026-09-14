mod ball;
mod bricks;
mod config;
mod game;
mod game_state;
mod paddle;

use crate::config::*;
use crate::game::Game;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("arkanoid")
        .build();

    let mut game = Game::new(&mut rl);

    rl.set_target_fps(90);
    while !rl.window_should_close() {
        game.update(&mut rl);

        // ----- drawing -----
        let mut d = rl.begin_drawing(&thread);
        game.draw(&mut d);
    }
}
