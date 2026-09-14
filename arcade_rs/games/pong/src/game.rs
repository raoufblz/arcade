use crate::ball::Ball;
use crate::config::*;
use crate::game_state::GameState;
use crate::paddle::Paddle;
use raylib::consts::DEG2RAD;
use raylib::prelude::*;

pub struct Game {
    pub state: GameState,
    pub right_paddle: Paddle,
    pub left_paddle: Paddle,
    pub score_right: i32,
    pub score_left: i32,
    pub ball: Ball,
}

impl Game {
    pub fn new(rl: &mut RaylibHandle) -> Self {
        Self {
            state: GameState::Countdown(4.0),
            score_right: 0,
            score_left: 0,
            ball: Ball::new(rl, Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0)),
            right_paddle: Paddle::new(
                SCREEN_WIDTH - PADDLE_OFFSET - PADDLE_WIDTH,
                (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0,
            ),
            left_paddle: Paddle::new(PADDLE_OFFSET, (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0),
        }
    }


    pub fn update(&mut self, rl: &mut RaylibHandle) {

	    if rl.is_key_pressed(KeyboardKey::KEY_P) {
	        self.state = match self.state {
	            GameState::Playing => GameState::Paused,
	            GameState::Paused => GameState::Playing,
	            _ => self.state,
	        };
	    }

	    if rl.is_key_pressed(KeyboardKey::KEY_R) {
	        self.score_left = 0;
	        self.score_right = 0;
	        self.left_paddle.reset(PADDLE_OFFSET, (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0);
	        self.right_paddle.reset(
	            SCREEN_WIDTH - PADDLE_OFFSET - PADDLE_WIDTH,
	            (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0,
	        );
	        self.ball.reset(rl);
	        self.state = GameState::Countdown(4.0);
	    }

	    let delta: f32 = rl.get_frame_time();

	    // ---- countdown tick ----
	    self.state = match self.state {
	        GameState::Countdown(t) if t - delta <= 0.0 => GameState::Playing,
	        GameState::Countdown(t) => GameState::Countdown(t - delta),
	        _ => self.state,
	    };

	    let can_move = matches!(self.state, GameState::Playing | GameState::Countdown(_));

	    if can_move {
	        // ---- Input ----
	        let dir_right: i32 = rl.is_key_down(KeyboardKey::KEY_DOWN) as i32
	            - rl.is_key_down(KeyboardKey::KEY_UP) as i32;
	        let dir_left: i32 = rl.is_key_down(KeyboardKey::KEY_S) as i32
	            - rl.is_key_down(KeyboardKey::KEY_W) as i32;

	        // ---- Update paddles ----
	        self.right_paddle.update(dir_right, delta);
	        self.left_paddle.update(dir_left, delta);
	    }

	    // ---- update only if Playing ----
	    if self.state == GameState::Playing {
	        // ---- Update ball ----
	        self.ball.update(delta);
	        self.ball.cap_speed();

	        // ---- Ball wall collisions ----
	        // left wall => right player scores
	        if self.ball.position.x < self.ball.radius {
	            self.score_right += 1;
	            self.ball.reset(rl);
	            self.state = GameState::Countdown(4.0);
	        }
	        // right wall => left player scores
	        if self.ball.position.x + self.ball.radius > SCREEN_WIDTH {
	            self.score_left += 1;
	            self.ball.reset(rl);
	            self.state = GameState::Countdown(4.0);
	        }
	        // Top wall
	        if self.ball.position.y < self.ball.radius {
	            self.ball.position.y = self.ball.radius;
	            self.ball.direction.y *= -1.0;
	            self.ball.speed *= SPEED_INCREMENT;
	            self.ball.cap_speed();
	        }
	        // Bottom wall
	        if self.ball.position.y + self.ball.radius > SCREEN_HEIGHT {
	            self.ball.position.y = SCREEN_HEIGHT - self.ball.radius;
	            self.ball.direction.y *= -1.0;
	            self.ball.speed *= SPEED_INCREMENT;
	            self.ball.cap_speed();
	        }

	        // ---- paddle collisions ----
	        let ball_center = self.ball.position;
	        let ball_radius = self.ball.radius;

	        // Right paddle
	        if self.right_paddle
	            .get_rect()
	            .check_collision_circle_rec(ball_center, ball_radius)
	        {
	            let paddle_rect = self.right_paddle.get_rect();
	            let hit_pos =
	                ((ball_center.y - paddle_rect.y) / paddle_rect.height).clamp(0.0, 1.0);
	            let angle_deg = (hit_pos - 0.5) * 2.0 * 75.0;
	            let angle_rad = angle_deg * DEG2RAD as f32;

	            self.ball.direction = Vector2::new(-angle_rad.cos(), angle_rad.sin()).normalized();
	            self.ball.speed *= SPEED_INCREMENT;
	            self.ball.cap_speed();
	            self.ball.position.x = paddle_rect.x - ball_radius;
	        }

	        // Left paddle
	        if self.left_paddle
	            .get_rect()
	            .check_collision_circle_rec(ball_center, ball_radius)
	        {
	            let paddle_rect = self.left_paddle.get_rect();
	            let hit_pos =
	                ((ball_center.y - paddle_rect.y) / paddle_rect.height).clamp(0.0, 1.0);
	            let angle_deg = (hit_pos - 0.5) * 2.0 * 75.0;
	            let angle_rad = angle_deg * DEG2RAD as f32;

	            self.ball.direction = Vector2::new(angle_rad.cos(), angle_rad.sin()).normalized();
	            self.ball.speed *= SPEED_INCREMENT;
	            self.ball.cap_speed();
	            self.ball.position.x = paddle_rect.x + paddle_rect.width + ball_radius;
	        }

	        // win condition
	        if self.score_right == WIN_SCORE || self.score_left == WIN_SCORE {
	            self.state = GameState::Win
	        }
	    } // end of Playing update
    }


    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::BLACK);

        // scores
        d.draw_text(
            &self.score_right.to_string(),
            SCREEN_WIDTH as i32 - 100,
            50,
            100,
            Color::WHITE,
        );
        d.draw_text(&self.score_left.to_string(), 50, 50, 100, Color::WHITE);

        // Paddles
        self.right_paddle.draw(d, Color::RED);
        self.left_paddle.draw(d, Color::BLUE);

        // Ball
        self.ball.draw(d);

        d.draw_fps(10, 10);

        // ---- countdown overlay ----
        if let GameState::Countdown(t) = self.state {
            let text = match t.ceil() as i32 {
                4 => "3",
                3 => "2",
                2 => "1",
                _ => "GO!",
            };
            let w = d.measure_text(text, 120) as f32;
            d.draw_text(
                text,
                ((SCREEN_WIDTH - w) / 2.0) as i32,
                (SCREEN_HEIGHT / 2.0 - 60.0) as i32,
                120,
                Color::WHITE,
            );
        }

        // Pause overlay
        if self.state == GameState::Paused {
            d.draw_rectangle(
                0,
                0,
                SCREEN_WIDTH as i32,
                SCREEN_HEIGHT as i32,
                Color::new(0, 0, 0, 100),
            );
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

        // later for ai enemy
        if self.state == GameState::GameOver {
            d.draw_rectangle(
                0,
                0,
                SCREEN_WIDTH as i32,
                SCREEN_HEIGHT as i32,
                Color::new(0, 0, 0, 100),
            );
            let loss_text = "game over";
            let loss_text_width = d.measure_text(loss_text, 80) as f32;

            d.draw_text(
                loss_text,
                ((SCREEN_WIDTH - loss_text_width) / 2.0) as i32,
                (SCREEN_HEIGHT / 2.0 - 40.0) as i32,
                80,
                Color::WHITE,
            );

            let restart_text = "press R to restart";
            let restart_text_width = d.measure_text(restart_text, 30) as f32;

            d.draw_text(
                restart_text,
                ((SCREEN_WIDTH - restart_text_width) / 2.0) as i32,
                (SCREEN_HEIGHT / 2.0 + 50.0) as i32,
                30,
                Color::LIGHTGRAY,
            );
        }

        if self.state == GameState::Win {
            d.draw_rectangle(
                0,
                0,
                SCREEN_WIDTH as i32,
                SCREEN_HEIGHT as i32,
                Color::new(0, 0, 0, 100),
            );
            let win_text = if self.score_left >= WIN_SCORE {
                "Left Player Wins!"
            } else {
                "Right Player Wins!"
            };

            let win_text_width = d.measure_text(win_text, 80) as f32;

            d.draw_text(
                win_text,
                ((SCREEN_WIDTH - win_text_width) / 2.0) as i32,
                (SCREEN_HEIGHT / 2.0 - 40.0) as i32,
                80,
                Color::WHITE,
            );

            let restart_text = "press R to restart";
            let restart_text_width = d.measure_text(restart_text, 30) as f32;

            d.draw_text(
                restart_text,
                ((SCREEN_WIDTH - restart_text_width) / 2.0) as i32,
                (SCREEN_HEIGHT / 2.0 + 50.0) as i32,
                30,
                Color::LIGHTGRAY,
            );
        }
    }
}
