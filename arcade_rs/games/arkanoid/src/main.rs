use raylib::prelude::*;

const 	SCREEN_WIDTH  :f32 = 1400.0;
const 	SCREEN_HEIGHT :f32 = 800.0;
const 	BALL_RADIUS   :f32 = 30.0;
const 	PADDLE_WIDTH  :f32 = 100.0;
const 	PADDLE_HEIGHT :f32 = 30.0;
const 	PADDLE_SPEED  :f32 = 400.0;

#[derive(PartialEq)]
enum GameState {
    Playing,
    Paused,
}

struct Paddle {
    pub position: Vector2,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
}

impl Paddle {
    pub fn new(position: Vector2) -> Self {
        Self {
            position,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
            speed: PADDLE_SPEED,
        }
    }

    pub fn update(&mut self, direction: i32, delta: f32, screen_width: f32) {
        self.position.x += direction as f32 * self.speed * delta;
        if self.position.x < 0.0 						{self.position.x = 0.0;}
        if self.position.x + self.width > screen_width 	{self.position.x = screen_width - self.width;}

    }
}

fn main() {
	let	mut	_speed_ball   :f32 = 500.0;
	let mut _score 	 	  :i32 = 0;
	let mut _lives 	 	  :i32 = 3;

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32 , SCREEN_HEIGHT as i32)
        .title("arkanoid")
        .build();

    let mut _game_state = GameState::Playing;

    let start_pos = Vector2::new((SCREEN_WIDTH - PADDLE_WIDTH) / 2.0, 700.0);
    let mut paddle = Paddle::new(start_pos);

    rl.set_target_fps(90);
    while !rl.window_should_close() {

    	let delta: f32 = rl.get_frame_time();
	    let paddle_direction :i32 = rl.is_key_down(KeyboardKey::KEY_RIGHT) as i32
								- rl.is_key_down(KeyboardKey::KEY_LEFT) as i32;

        paddle.update(paddle_direction, delta, SCREEN_WIDTH);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        let paddle = Rectangle::new(paddle.position.x, paddle.position.y, PADDLE_WIDTH, PADDLE_HEIGHT);
        d.draw_rectangle_rec(paddle, Color::new(255, 0, 0, 255));

    }
}