use raylib::prelude::*;

const CENTER_X: i32 = 320;
const CENTER_Y: i32 = 240;
const RADIUS: f32 = 10.0;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Pong")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_circle(CENTER_X, CENTER_Y, RADIUS, Color::CYAN);
    }
}
