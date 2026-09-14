use crate::ball::Ball;
use crate::config::*;
use crate::game_state::GameState;
use crate::paddle::Paddle;
use crate::bricks::Brick;

use raylib::consts::DEG2RAD;
use raylib::prelude::*;


pub struct Game {
	pub game_state: GameState,
	pub ball: Ball,
	pub paddle: Paddle,
	pub bricks: Vec<Brick>,
	pub score: i32,
	pub lives: i32,
}

impl Game {
	pub fn new(rl: &mut RaylibHandle) -> Self {
		Self{
			game_state: GameState::Countdown(4.0),
			ball: Ball::new(
    			rl,
    			Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
    		),
			paddle: Paddle::new(Vector2::new((SCREEN_WIDTH - PADDLE_WIDTH) / 2.0, 700.0)),
			bricks: bricks,
			score: 0,
			lives: INITIAL_LIVES,
		}
	}


	pub fn update(&mut self, rl: &mut RaylibHandle) {

		if rl.is_key_pressed(KeyboardKey::KEY_P) {
            self.game_state = match self.game_state {
                GameState::Playing => GameState::Paused,
                GameState::Paused => GameState::Playing,
                _ => self.game_state,
            };
        }

        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            self.paddle.reset((SCREEN_WIDTH - PADDLE_WIDTH) / 2.0, 700.0);
            self.ball.reset(&mut rl);
            self.game_state = GameState::Countdown(4.0);
            self.lives = INITIAL_LIVES;
            self.score = 0;
            for brick in &mut self.bricks {
                brick.broken = false;
            }
        }

        let delta: f32 = rl.get_frame_time();

        // ---- countdown tick ----
        self.game_state = match self.game_state {
            GameState::Countdown(t) if t - delta <= 0.0 => GameState::Playing,
            GameState::Countdown(t) => GameState::Countdown(t - delta),
            other => other,
        };
        let can_move = matches!(self.game_state, GameState::Playing | GameState::Countdown(_));

        if can_move {
            let paddle_direction: i32 = rl.is_key_down(KeyboardKey::KEY_RIGHT) as i32
                - rl.is_key_down(KeyboardKey::KEY_LEFT) as i32;

            self.paddle.update(paddle_direction, delta, SCREEN_WIDTH);
        }

        if self.game_state == GameState::Playing {
            self.ball.update(delta);
            self.ball.cap_speed();

            // ---- ball/wall collisions ----
            // left wall
            if self.ball.position.x < self.ball.radius {
                self.ball.position.x = self.ball.radius;
                self.ball.direction.x *= -1.0;
                self.ball.cap_speed();
            }
            // right wall
            if self.ball.position.x + self.ball.radius > SCREEN_WIDTH {
                self.ball.position.x = SCREEN_WIDTH - self.ball.radius;
                self.ball.direction.x *= -1.0;
                self.ball.cap_speed();
            }
            // Top wall
            if self.ball.position.y < self.ball.radius {
                self.ball.position.y = self.ball.radius;
                self.ball.direction.y *= -1.0;
                self.ball.cap_speed();
            }
            // Bottom wall
            if self.ball.position.y + self.ball.radius > SCREEN_HEIGHT {
                self.lives -= 1;
                if self.lives <= 0 {
                self.game_state = GameState::GameOver;
                } else {
                    self.paddle.reset((SCREEN_WIDTH - self.paddle.width) / 2.0, 700.0);
                    self.ball.reset(&mut rl);
                    self.game_state = GameState::Countdown(4.0);
                }
            }

            // ---- paddle collisions ----
            if self.paddle
                .get_rect()
                .check_collision_circle_rec(self.ball.position, self.ball.radius)
            {
                let paddle_rect = self.paddle.get_rect();
                let hit_pos =
                    ((self.ball.position.x - paddle_rect.x) / paddle_rect.width).clamp(0.0, 1.0);
                let angle_deg = (hit_pos - 0.5) * 2.0 * 75.0; // 75 max
                let angle_rad = angle_deg * DEG2RAD as f32;

                self.ball.direction = Vector2::new(angle_rad.sin(), -angle_rad.cos()).normalized();
                self.ball.speed *= SPEED_INCREMENT;
                self.ball.cap_speed();
                self.ball.position.y = paddle_rect.y - self.ball.radius;
            }

            // brick collisions
            for brick in &mut bricks {
                if !brick.is_broken()
                    && brick
                        .get_rect()
                        .check_collision_circle_rec(self.ball.position, self.ball.radius)
                {
                    brick.do_break();
                    self.score += 1;

                    // find which side was hit
                    let brick_rect = brick.get_rect();
                    let overlap_x = if self.ball.position.x < brick_rect.x + brick_rect.width / 2.0 {
                        (self.ball.position.x + self.ball.radius) - brick_rect.x
                    } else {
                        (brick_rect.x + brick_rect.width) - (self.ball.position.x - self.ball.radius)
                    };
                    let overlap_y = if self.ball.position.y < brick_rect.y + brick_rect.height / 2.0 {
                        (self.ball.position.y + self.ball.radius) - brick_rect.y
                    } else {
                        (brick_rect.y + brick_rect.height) - (self.ball.position.y - self.ball.radius)
                    };

                    // bounce
                    if overlap_x < overlap_y {
                        // left or right side hit
                        self.ball.direction.x *= -1.0;
                        if self.ball.direction.x < 0.0 {
                            self.ball.position.x = brick_rect.x - self.ball.radius;
                        } else {
                            self.ball.position.x = brick_rect.x + brick_rect.width + self.ball.radius;
                        }
                    } else {
                        // top or bottom side hit
                        self.ball.direction.y *= -1.0;
                        if self.ball.direction.y < 0.0 {
                            self.ball.position.y = brick_rect.y - self.ball.radius;
                        } else {
                            self.ball.position.y = brick_rect.y + brick_rect.height + self.ball.radius;
                        }
                    }

                    self.ball.speed *= SPEED_INCREMENT;
                    self.ball.cap_speed();

                    break;
                }
            }

            // after brick collision loop
            let all_broken = self.bricks.iter().all(|b| b.is_broken());
            if all_broken {
                self.game_state = GameState::Win;
            }
        }

	}


	pub fn draw(&self, d: &mut RaylibDrawHandle) {
		d.clear_background(Color::BLACK);
        d.draw_fps(10, 10);
        self.paddle.draw(&mut d, Color::new(255, 0, 0, 255));
        self.ball.draw(&mut d);

        // drawing bricks
        for brick in &bricks {
            if !brick.is_broken() {
                // assign colors based on row
                let color = if brick.position.y < 125.0 {
                    Color::new(255, 0, 0, 255) // red
                } else if brick.position.y < 250.0 {
                    Color::new(0, 255, 0, 255) // green
                } else {
                    Color::new(0, 0, 255, 255) // blue
                };
                brick.draw(&mut d, color);
            }
        }

        d.draw_text(&format!("Lives: {}", self.lives), 20, 12, 30, Color::WHITE);
        let score_text = format!("Score: {}", self.score);
        let score_w = d.measure_text(&score_text, 30);
        d.draw_text(
            &score_text,
            SCREEN_WIDTH as i32 - score_w - 20,
            12,
            30,
            Color::WHITE,
        );

        // ---- countdown overlay ----
        if let GameState::Countdown(t) = self.game_state {
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
        if self.game_state == GameState::Paused {
            d.draw_rectangle(
                0,
                0,
                SCREEN_WIDTH as i32,
                SCREEN_HEIGHT as i32,
                Color::new(0, 0, 0, 100),
            );
            let pause_text = "PAUSED";
            let pause_text_width = d.measure_text(pause_text, 80) as f32;
            d.draw_text(
                pause_text,
                ((SCREEN_WIDTH - pause_text_width) / 2.0) as i32,
                (SCREEN_HEIGHT / 2.0 - 40.0) as i32,
                80,
                Color::WHITE,
            );
            let restart_text = "Press P to resume, R to restart";
            let restart_text_width = d.measure_text(restart_text, 30) as f32;
            d.draw_text(
                restart_text,
                ((SCREEN_WIDTH - restart_text_width) / 2.0) as i32,
                (SCREEN_HEIGHT / 2.0 + 50.0) as i32,
                30,
                Color::LIGHTGRAY,
            );
        }

        if self.game_state == GameState::GameOver {
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

        if self.game_state == GameState::Win {
            d.draw_rectangle(
                0,
                0,
                SCREEN_WIDTH as i32,
                SCREEN_HEIGHT as i32,
                Color::new(0, 0, 0, 100),
            );
            let win_text = "you win!";
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
