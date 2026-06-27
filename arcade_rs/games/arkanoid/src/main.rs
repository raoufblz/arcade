use raylib::prelude::*;


#[derive(PartialEq)]
enum GameState {
    Playing,
    Paused,
}

fn main() {
	const 	SCREEN_WIDTH  :f32 = 1400.0;
    const 	SCREEN_HEIGHT :f32 = 800.0;
    const 	SPEED_PADDLE  :f32 = 400.0;
    let	mut	_speed_ball    :f32 = 500.0;
    const 	PADDLE_WIDTH  :f32 = 100.0;
    const 	PADDLE_HEIGHT :f32 = 30.0;
    const 	BALL_RADIUS   :f32 = 30.0;
    let mut _score 	 	  :i32 = 0;
    let mut _lives 	 	  :i32 = 3;

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32 , SCREEN_HEIGHT as i32)
        .title("arkanoid")
        .build();

    let mut _game_state = GameState::Playing;

    let mut paddle_pos = Vector2::new((SCREEN_WIDTH - PADDLE_WIDTH) / 2.0, 700.0);

    rl.set_target_fps(90);
    while !rl.window_should_close() {

    	let delta: f32 = rl.get_frame_time();
	    let paddle_direction :i32 = rl.is_key_down(KeyboardKey::KEY_RIGHT) as i32
								- rl.is_key_down(KeyboardKey::KEY_LEFT) as i32;

	    paddle_pos.x += paddle_direction as f32 * SPEED_PADDLE * delta;

        if paddle_pos.x < 0.0 						    {paddle_pos.x = 0.0;}
        if paddle_pos.x + PADDLE_WIDTH > SCREEN_WIDTH 	{paddle_pos.x = SCREEN_WIDTH - PADDLE_WIDTH;}


        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        let paddle = Rectangle::new(paddle_pos.x, paddle_pos.y, PADDLE_WIDTH, PADDLE_HEIGHT);
        d.draw_rectangle_rec(paddle, Color::new(255, 0, 0, 255));

    }
}