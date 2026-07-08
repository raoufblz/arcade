mod config;
mod paddle;
mod ball;
mod bricks;
mod game_state;


use raylib::prelude::*;
use raylib::consts::DEG2RAD;
use crate::config::*;
use crate::game_state::GameState;
use crate::paddle::Paddle;
use crate::ball::Ball;
use crate::bricks::Brick;


fn main() {
	let mut score 	 	  :i32 = 0;
	let mut lives 	 	  :i32 = INITIAL_LIVES;

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32 , SCREEN_HEIGHT as i32)
        .title("arkanoid")
        .build();

    let mut game_state = GameState::Playing;

    let start_pos = Vector2::new((SCREEN_WIDTH - PADDLE_WIDTH) / 2.0, 700.0);

    let mut paddle = Paddle::new(start_pos);


    let mut angle = 0;
    while angle % 90 == 0 {
        angle = rl.get_random_value(15..165);
    }

    let radians: f32 = angle as f32 * DEG2RAD as f32;
    let direction = Vector2::new(radians.cos(), radians.sin());

    let mut ball = Ball::new(Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0), direction);

   	// ---- the bricks ----
    let mut bricks = Vec::new();
    let brick_padding = 15.0;
    let total_bricks_width = BRICK_COLS as f32 * (BRICK_WIDTH + brick_padding) - brick_padding;
    let start_x = (SCREEN_WIDTH - total_bricks_width) / 2.0;
    let start_y = 50.0;

    for row in 0..BRICK_ROWS {
        for col in 0..BRICK_COLS {
            let x = start_x + col as f32 * (BRICK_WIDTH + brick_padding);
            let y = start_y + row as f32 * (BRICK_HEIGHT + brick_padding);
            bricks.push(Brick::new(Vector2::new(x, y)));
        }
    }

    rl.set_target_fps(90);
    while !rl.window_should_close() {

	   	if rl.is_key_pressed(KeyboardKey::KEY_P) {
                game_state = match game_state {
                    GameState::Playing => GameState::Paused,
                    GameState::Paused => GameState::Playing,
                    _ => game_state,
                };
            }

        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            paddle.reset((SCREEN_WIDTH - PADDLE_WIDTH) / 2.0, 700.0);
            ball.reset(&mut rl);
            game_state = GameState::Playing;
            lives = INITIAL_LIVES;
            score = 0;
            for brick in &mut bricks {
                brick.broken = false;
            }
        }

    	let delta: f32 = rl.get_frame_time();

     	if game_state == GameState::Playing {
		    let paddle_direction :i32 = rl.is_key_down(KeyboardKey::KEY_RIGHT) as i32
									- rl.is_key_down(KeyboardKey::KEY_LEFT) as i32;

	        paddle.update(paddle_direction, delta, SCREEN_WIDTH);

	        ball.update(delta);
	        ball.cap_speed();

	        // ---- ball/wall collisions ----
	        // left wall
	        if ball.position.x < ball.radius {
		        ball.position.x = ball.radius;
	            ball.direction.x *= -1.0;
	            ball.cap_speed();
	        }
	        // right wall
	        if ball.position.x + ball.radius > SCREEN_WIDTH {
		        ball.position.x = SCREEN_WIDTH - ball.radius;
	            ball.direction.x *= -1.0;
	            ball.cap_speed();
	        }
	        // Top wall
	        if ball.position.y < ball.radius {
	            ball.position.y = ball.radius;
	            ball.direction.y *= -1.0;
	            ball.cap_speed();
	        }
	        // Bottom wall
	        if ball.position.y + ball.radius > SCREEN_HEIGHT {
				lives -= 1;
				if lives <= 0 {
			        game_state = GameState::GameOver;
			    } else {
					paddle.reset((SCREEN_WIDTH - paddle.width) / 2.0, 700.0);
		            ball.reset(&mut rl);
		            game_state = GameState::Playing;
				}
			}

			// ---- paddle collisions ----
			if paddle.get_rect().check_collision_circle_rec(ball.position, ball.radius) {
			    let paddle_rect = paddle.get_rect();
			    let hit_pos = ((ball.position.x - paddle_rect.x) / paddle_rect.width).clamp(0.0, 1.0);
			    let angle_deg = (hit_pos - 0.5) * 2.0 * 75.0; 		// 75 max
			    let angle_rad = angle_deg * DEG2RAD as f32;

			    ball.direction = Vector2::new(angle_rad.sin(), -angle_rad.cos()).normalized();
			    ball.speed *= SPEED_INCREMENT;
			    ball.cap_speed();
			    ball.position.y = paddle_rect.y - ball.radius;
			}


			// brick collisions
			for brick in &mut bricks {
			    if !brick.is_broken() {
			        if brick.get_rect().check_collision_circle_rec(ball.position, ball.radius) {
			            brick.do_break();
			            score += 1;

			            // find which side was hit
			            let brick_rect = brick.get_rect();
			            let overlap_x = if ball.position.x < brick_rect.x + brick_rect.width / 2.0 {
			                (ball.position.x + ball.radius) - brick_rect.x
			            } else {
			                (brick_rect.x + brick_rect.width) - (ball.position.x - ball.radius)
			            };
			            let overlap_y = if ball.position.y < brick_rect.y + brick_rect.height / 2.0 {
			                (ball.position.y + ball.radius) - brick_rect.y
			            } else {
			                (brick_rect.y + brick_rect.height) - (ball.position.y - ball.radius)
			            };

			            // bounce
			            if overlap_x < overlap_y {
			                // left or right side hit
			                ball.direction.x *= -1.0;
			                if ball.direction.x < 0.0 {
			                    ball.position.x = brick_rect.x - ball.radius;
			                } else {
			                    ball.position.x = brick_rect.x + brick_rect.width + ball.radius;
			                }
			            } else {
			                // top or bottom side hit
			                ball.direction.y *= -1.0;
			                if ball.direction.y < 0.0 {
			                    ball.position.y = brick_rect.y - ball.radius;
			                } else {
			                    ball.position.y = brick_rect.y + brick_rect.height + ball.radius;
			                }
			            }

			            ball.speed *= SPEED_INCREMENT;
			            ball.cap_speed();

			            break;
			        }
			    }
			}

			// after brick collision loop
		    let all_broken = bricks.iter().all(|b| b.is_broken());
		    if all_broken {
			   game_state = GameState::Win;
		    }
      	}



        // ----- drawing -----
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_fps(10, 10);
        paddle.draw(&mut d, Color::new(255, 0, 0, 255));
        ball.draw(&mut d);

        // drawing bricks
        for brick in &bricks {
            if !brick.is_broken() {
                // assign colors based on row
                let color = if brick.position.y < 125.0 {
                    Color::new(255, 0, 0, 255)     // red
                } else if brick.position.y < 250.0 {
                    Color::new(0, 255, 0, 255)     // green
                } else {
                    Color::new(0, 0, 255, 255)     // blue
                };
                brick.draw(&mut d, color);
            }
        }

        d.draw_text(&lives.to_string(), 50, 50, 100, Color::WHITE);
        d.draw_text(&score.to_string(), SCREEN_WIDTH as i32 - 100, 50, 100, Color::WHITE);

        // Pause overlay
        if game_state == GameState::Paused {
            d.draw_rectangle(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, Color::new(0, 0, 0, 100));
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

        if game_state == GameState::GameOver {
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

        if game_state == GameState::Win {
	        d.draw_rectangle(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, Color::new(0, 0, 0, 100));
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