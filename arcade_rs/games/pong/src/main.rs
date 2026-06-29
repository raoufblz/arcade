use raylib::consts::DEG2RAD;
use raylib::prelude::*;

const SCREEN_WIDTH		: f32 = 1600.0;
const SCREEN_HEIGHT		: f32 = 900.0;
const PADDLE_SPEED		: f32 = 400.0;
const BALL_SPEED		: f32 = 500.0;
const MAX_SPEED			: f32 = 1500.0;
const RECT_WIDTH		: f32 = 35.0;
const RECT_HEIGHT		: f32 = 100.0;
const BALL_RAD			: f32 = 30.0;
const PADDLE_OFFSET		: f32 = 50.0;
const SPEED_INCREMENT	: f32 = 1.01;

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
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vector2::new(x, y),
            width: RECT_WIDTH,
            height: RECT_HEIGHT,
            speed: PADDLE_SPEED,
        }
    }

    pub fn update(&mut self, direction: i32, delta: f32) {
        self.position.y += direction as f32 * self.speed * delta;
        self.position.y = self.position.y.clamp(0.0, SCREEN_HEIGHT - self.height);
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
            radius: BALL_RAD,
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

fn main() {
    let mut score_left: i32 = 0;
    let mut score_right: i32 = 0;
    let mut state = GameState::Playing;

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Pong")
        .build();

    // ---- Create paddles ----
    let mut left_paddle = Paddle::new(PADDLE_OFFSET, (SCREEN_HEIGHT - RECT_HEIGHT) / 2.0);
    let mut right_paddle = Paddle::new(
        SCREEN_WIDTH - PADDLE_OFFSET - RECT_WIDTH,
        (SCREEN_HEIGHT - RECT_HEIGHT) / 2.0,
    );

    // ---- create ball with random initial direction ----
    let mut angle = 0;
    while angle % 90 == 0 || (angle > 75 && angle < 105) || (angle > 255 && angle < 285) {
        angle = rl.get_random_value(1..360);
    }
    let radians: f32 = angle as f32 * DEG2RAD as f32;
    let direction = Vector2::new(radians.cos(), radians.sin());
    let mut ball = Ball::new(Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0), direction);

    rl.set_target_fps(90);

    while !rl.window_should_close() {
        // ---- input handling ----
        if rl.is_key_pressed(KeyboardKey::KEY_P) {
            state = match state {
                GameState::Playing => GameState::Paused,
                GameState::Paused => GameState::Playing,
            };
        }

        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            score_left = 0;
            score_right = 0;
            left_paddle.reset(PADDLE_OFFSET, (SCREEN_HEIGHT - RECT_HEIGHT) / 2.0);
            right_paddle.reset(
                SCREEN_WIDTH - PADDLE_OFFSET - RECT_WIDTH,
                (SCREEN_HEIGHT - RECT_HEIGHT) / 2.0,
            );
            ball.reset(&mut rl);
            state = GameState::Playing;
        }

        let delta: f32 = rl.get_frame_time();

        // ---- update only if Playing ----
        if state == GameState::Playing {
            // ---- Input ----
            let dir_right: i32 =
                rl.is_key_down(KeyboardKey::KEY_DOWN) as i32 - rl.is_key_down(KeyboardKey::KEY_UP) as i32;
            let dir_left: i32 =
                rl.is_key_down(KeyboardKey::KEY_S) as i32 - rl.is_key_down(KeyboardKey::KEY_W) as i32;

            // ---- Update paddles ----
            right_paddle.update(dir_right, delta);
            left_paddle.update(dir_left, delta);

            // ---- Update ball ----
            ball.update(delta);
            ball.cap_speed();

            // ---- Ball wall collisions ----
            // left wall => right player scores
            if ball.position.x < ball.radius {
                score_right += 1;
                ball.reset(&mut rl);
            }
            // right wall => left player scores
            if ball.position.x + ball.radius > SCREEN_WIDTH {
                score_left += 1;
                ball.reset(&mut rl);
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

            // Right paddle
            if right_paddle.get_rect().check_collision_circle_rec(ball_center, ball_radius) {
                ball.direction.x *= -1.0;
                ball.speed *= SPEED_INCREMENT;
                ball.cap_speed();
                ball.position.x = right_paddle.position.x - ball_radius;
            }

            // Left paddle
            if left_paddle.get_rect().check_collision_circle_rec(ball_center, ball_radius) {
                ball.direction.x *= -1.0;
                ball.speed *= SPEED_INCREMENT;
                ball.cap_speed();
                ball.position.x = left_paddle.position.x + left_paddle.width + ball_radius;
            }
        } // end of Playing update

        // ---- drawing ----
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // scores
        d.draw_text(&score_right.to_string(), SCREEN_WIDTH as i32 - 100, 50, 100, Color::WHITE);
        d.draw_text(&score_left.to_string(), 50, 50, 100, Color::WHITE);

        // Paddles
        right_paddle.draw(&mut d, Color::RED);
        left_paddle.draw(&mut d, Color::BLUE);

        // Ball
        ball.draw(&mut d);

        d.draw_fps(10, 10);

        // Pause overlay
        if state == GameState::Paused {
            d.draw_rectangle(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, Color::new(0, 0, 0, 100));
            d.draw_text(
                "PAUSED",
                (SCREEN_WIDTH / 2.0 - 140.0) as i32,
                (SCREEN_HEIGHT / 2.0 - 40.0) as i32,
                80,
                Color::WHITE,
            );
            d.draw_text(
                "Press P to resume, R to restart",
                (SCREEN_WIDTH / 2.0 - 230.0) as i32,
                (SCREEN_HEIGHT / 2.0 + 50.0) as i32,
                30,
                Color::LIGHTGRAY,
            );
        }
    }
}