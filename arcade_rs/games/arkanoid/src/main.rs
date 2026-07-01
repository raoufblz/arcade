use raylib::prelude::*;

const 	SCREEN_WIDTH  	:f32 	= 1400.0;
const 	SCREEN_HEIGHT 	:f32 	= 800.0;
const 	PADDLE_WIDTH  	:f32 	= 100.0;
const 	PADDLE_HEIGHT 	:f32 	= 30.0;
const 	PADDLE_SPEED  	:f32 	= 400.0;
const 	BALL_RADIUS   	:f32 	= 20.0;
const	BALL_SPEED	  	:f32 	= 500.0;
const 	MAX_SPEED	  	:f32 	= 1500.0;
const 	SPEED_INCREMENT	:f32 	= 1.01;

const 	BRICK_WIDTH  	:f32 	= 80.0;
const 	BRICK_HEIGHT 	:f32 	= 30.0;
const	BRICK_ROWS		:i32	= 5;
const	BRICK_COLS		:i32	= 10;


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
		self.position.x = self.position.x.clamp(0.0, screen_width - self.width);
    }

	pub fn reset(&mut self, x: f32, y: f32) {
        self.position = Vector2::new(x, y);
    }

    pub fn get_rect(&self) -> Rectangle {
        Rectangle::new(self.position.x, self.position.y, self.width, self.height)
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, color: Color) {
        d.draw_rectangle_rec(self.get_rect(), color);
    }
}


struct Ball {
	pub position: Vector2,
	pub direction: Vector2,
	pub speed: f32,
	pub radius: f32,
}

impl Ball {
    pub fn new(position: Vector2, direction: Vector2) -> Self {
        Self {
            position,
            direction,
            speed: BALL_SPEED,
            radius: BALL_RADIUS,
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.position.x += self.direction.x * self.speed * delta;
        self.position.y += self.direction.y * self.speed * delta;
    }

    pub fn reset(&mut self, rl: &mut RaylibHandle) {
        self.position = Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0);
        self.speed = BALL_SPEED;

        let mut angle = 0;
        while angle % 90 == 0 || (angle > 75 && angle < 105) || (angle > 255 && angle < 285) {
            angle = rl.get_random_value(1..360);
        }
        let radians: f32 = (angle as f32) * DEG2RAD as f32;
        self.direction = Vector2::new(radians.cos(), radians.sin());
    }

    pub fn cap_speed(&mut self) {
        if self.speed > MAX_SPEED {
            self.speed = MAX_SPEED;
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle_v(self.position, self.radius, Color::new(255, 255, 0, 255));
    }
}


struct Brick {
    pub position: Vector2,
    pub width: f32,
    pub height: f32,
    pub broken: bool,
}

impl Brick {
    pub fn new(position: Vector2) -> Self {
        Self {
            position,
            width: BRICK_WIDTH,
            height: BRICK_HEIGHT,
            broken: false,
        }
    }

   	// the loop breaks them, bricks don t update, we ll see

	pub fn reset(&mut self, x: f32, y: f32) {
		self.position = Vector2::new(x, y);
		self.broken = false;
	}

	pub fn is_broken(&self) -> bool {
		self.broken
	}

	pub fn do_break(&mut self) {
		self.broken = true;
	}

    pub fn get_rect(&self) -> Rectangle {
        Rectangle::new(self.position.x, self.position.y, self.width, self.height)
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, color: Color) {
        d.draw_rectangle_rec(self.get_rect(), color);
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

    rl.set_target_fps(90);
    let mut _game_state = GameState::Playing;

    let start_pos = Vector2::new((SCREEN_WIDTH - PADDLE_WIDTH) / 2.0, 700.0);

    let mut paddle = Paddle::new(start_pos);


    let mut angle = 0;
    while angle % 90 == 0 {
        angle = rl.get_random_value(15..165);
    }

    let radians: f32 = angle as f32 * DEG2RAD as f32;
    let direction = Vector2::new(radians.cos(), radians.sin());

    let mut ball = Ball::new(Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0), direction);

    rl.set_target_fps(90);
    while !rl.window_should_close() {

    	let delta: f32 = rl.get_frame_time();
	    let paddle_direction :i32 = rl.is_key_down(KeyboardKey::KEY_RIGHT) as i32
								- rl.is_key_down(KeyboardKey::KEY_LEFT) as i32;

        paddle.update(paddle_direction, delta, SCREEN_WIDTH);

        // ----- experimental -----
        ball.update(delta);
        ball.cap_speed();

        // ---- Ball wall collisions ----
        // left wall
        if ball.position.x < ball.radius {
	        ball.position.x = ball.radius;
            ball.direction.x *= -1.0;
            ball.speed *= SPEED_INCREMENT;
            ball.cap_speed();
        }
        // right wall
        if ball.position.x + ball.radius > SCREEN_WIDTH {
	        ball.position.x = SCREEN_WIDTH - ball.radius;
            ball.direction.x *= -1.0;
            ball.speed *= SPEED_INCREMENT;
            ball.cap_speed();
        }
        // Top wall
        if ball.position.y < ball.radius {
            ball.position.y = ball.radius;
            ball.direction.y *= -1.0;
            ball.speed *= SPEED_INCREMENT;
            ball.cap_speed();
        }
        // Bottom wall
        if ball.position.y + ball.radius > SCREEN_HEIGHT {
            ball.position.y = SCREEN_HEIGHT - ball.radius;
            ball.direction.y *= -1.0;
            ball.speed *= SPEED_INCREMENT;
            ball.cap_speed();
        }

        // ---- paddle collisions ----
        let ball_center = ball.position;
        let ball_radius = ball.radius;

        // paddle
        if paddle.get_rect().check_collision_circle_rec(ball_center, ball_radius) {
            ball.direction.y *= -1.0;
            ball.speed *= SPEED_INCREMENT;
            ball.cap_speed();
            ball.position.y = paddle.position.y - ball_radius;
        }

        // ----- end of experimental -----

        // ----- drawing -----
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_fps(10, 10);
        paddle.draw(&mut d, Color::new(255, 0, 0, 255));
        ball.draw(&mut d);


    }
}