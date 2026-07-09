mod config;
mod paddle;
mod ball;
mod game_state;

use crate::config::*;
use crate::game_state::GameState;
use crate::paddle::Paddle;
use crate::ball::Ball;

use raylib::consts::DEG2RAD;
use raylib::prelude::*;


fn main() {
    let mut score_left: i32 = 0;
    let mut score_right: i32 = 0;
    let mut state = GameState::Playing;

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Pong")
        .build();

    // ---- Create paddles ----
    let mut left_paddle = Paddle::new(PADDLE_OFFSET, (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0);
    let mut right_paddle = Paddle::new(
        SCREEN_WIDTH - PADDLE_OFFSET - PADDLE_WIDTH,
        (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0,
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
                _ => state,
            };
        }

        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            score_left = 0;
            score_right = 0;
            left_paddle.reset(PADDLE_OFFSET, (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0);
            right_paddle.reset(
                SCREEN_WIDTH - PADDLE_OFFSET - PADDLE_WIDTH,
                (SCREEN_HEIGHT - PADDLE_HEIGHT) / 2.0,
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
                let paddle_rect = right_paddle.get_rect();
                let hit_pos = ((ball_center.y - paddle_rect.y) / paddle_rect.height).clamp(0.0, 1.0);
                let angle_deg = (hit_pos - 0.5) * 2.0 * 75.0;
                let angle_rad = angle_deg * DEG2RAD as f32;

                ball.direction = Vector2::new(-angle_rad.cos(), angle_rad.sin()).normalized();
                ball.speed *= SPEED_INCREMENT;
                ball.cap_speed();
                ball.position.x = paddle_rect.x - ball_radius;
            }

            // Left paddle
            if left_paddle.get_rect().check_collision_circle_rec(ball_center, ball_radius) {
                let paddle_rect = left_paddle.get_rect();
                let hit_pos = ((ball_center.y - paddle_rect.y) / paddle_rect.height).clamp(0.0, 1.0);
                let angle_deg = (hit_pos - 0.5) * 2.0 * 75.0;
                let angle_rad = angle_deg * DEG2RAD as f32;

                ball.direction = Vector2::new(angle_rad.cos(), angle_rad.sin()).normalized();
                ball.speed *= SPEED_INCREMENT;
                ball.cap_speed();
                ball.position.x = paddle_rect.x + paddle_rect.width + ball_radius;
            }

            // win condition
            if score_right == WIN_SCORE || score_left == WIN_SCORE {
            	state = GameState::Win
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

        // later for ai enemy
        if state == GameState::GameOver {
	        d.draw_rectangle(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, Color::new(0, 0, 0, 100));
			let loss_text = "game over";
			let loss_text_width = d.measure_text(loss_text, 80) as f32;

			d.draw_text(
				loss_text,
				((SCREEN_WIDTH - loss_text_width) / 2.0) as i32,
				(SCREEN_HEIGHT / 2.0 - 40.0) as i32,
				80,
				Color::WHITE
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

        if state == GameState::Win {
	        d.draw_rectangle(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, Color::new(0, 0, 0, 100));
			let win_text = if score_left >= WIN_SCORE {
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