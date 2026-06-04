use raylib::prelude::*;
use raylib::consts::DEG2RAD;

fn main() {
	const 	SCREEN_WIDTH	:f32   	= 1600.0;
	const 	SCREEN_HEIGHT	:f32  	= 900.0;
	const 	SPEED 			:f32   	= 400.0;
	let mut speed_ball		:f32  	= 500.0;
	const   MAX_SPEED		:f32  	= 1500.0;
	const 	RECT_WIDTH    	:f32	= 35.0;
	const 	RECT_HEIGHT    	:f32	= 100.0;
	const 	BALL_RAD    	:f32	= 30.0;
	let mut score_left		:i32	= 0;
	let mut score_right		:i32	= 0;
	const 	PADDLE_OFFSET   :f32	= 50.0;

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Pong")
        .build();

    let mut angle = 0;
    while angle % 90 == 0 || (angle > 75 && angle < 105) || (angle > 255 && angle < 285) {angle = rl.get_random_value(1..360);}
    let radians: f32 = angle as f32 * DEG2RAD as f32;
    let mut direction_ball = Vector2::new(radians.cos(), radians.sin());

    rl.set_target_fps(90);
    let mut position_pdl_right = Vector2::new(SCREEN_WIDTH - PADDLE_OFFSET - RECT_WIDTH, (SCREEN_HEIGHT - RECT_HEIGHT)/2.0);
    let mut position_pdl_left = Vector2::new(PADDLE_OFFSET, (SCREEN_HEIGHT - RECT_HEIGHT)/2.0);


    let mut position_ball = Vector2::new((SCREEN_WIDTH)/ 2.0 , (SCREEN_HEIGHT)/ 2.0);

    fn cap_speed(speed: &mut f32, max_speed: f32) {
        if *speed > max_speed {
            *speed = max_speed;
        }
    }

    fn reset_ball(rl: &mut RaylibHandle, pos: &mut Vector2, speed: &mut f32, dir: &mut Vector2, screen_w: f32, screen_h: f32){
        *pos = Vector2::new(screen_w / 2.0, screen_h / 2.0);
        *speed = 500.0;

        let mut angle = 0;
        while angle % 90 == 0 || (angle > 75 && angle < 105) || (angle > 255 && angle < 285) {angle = rl.get_random_value(1..360);}
        let radians: f32 = (angle as f32) * DEG2RAD as f32;

        *dir = Vector2::new(radians.cos(), radians.sin());
    }

    while !rl.window_should_close() {

	   	let delta       :f32 = rl.get_frame_time();
        let direction_1 :i32 = rl.is_key_down(KeyboardKey::KEY_DOWN) as i32 - rl.is_key_down(KeyboardKey::KEY_UP) as i32;
        let direction_2 :i32 = rl.is_key_down(KeyboardKey::KEY_S) as i32 - rl.is_key_down(KeyboardKey::KEY_W) as i32;

        position_pdl_right.y += direction_1 as f32 * SPEED * delta;

        //keeping rect inside window
        if position_pdl_right.y < 0.0 						  {position_pdl_right.y = 0.0;}
        if position_pdl_right.y + RECT_HEIGHT > SCREEN_HEIGHT {position_pdl_right.y =SCREEN_HEIGHT - RECT_HEIGHT;}

        position_pdl_left.y += direction_2 as f32 * SPEED * delta;

        //keeping rect inside window
        if position_pdl_left.y < 0.0 						  {position_pdl_left.y = 0.0;}
        if position_pdl_left.y + RECT_HEIGHT > SCREEN_HEIGHT  {position_pdl_left.y = SCREEN_HEIGHT - RECT_HEIGHT;}

        cap_speed(&mut speed_ball, MAX_SPEED);
        position_ball.x += direction_ball.x * speed_ball * delta;
        position_ball.y += direction_ball.y * speed_ball * delta;

//==================== start of ball logic --------
        if position_ball.x < BALL_RAD{
        score_right += 1;
        reset_ball(&mut rl, &mut position_ball, &mut speed_ball, &mut direction_ball, SCREEN_WIDTH, SCREEN_HEIGHT);
        }

        if position_ball.x + BALL_RAD > SCREEN_WIDTH {
        score_left += 1;
        reset_ball(&mut rl, &mut position_ball, &mut speed_ball, &mut direction_ball, SCREEN_WIDTH, SCREEN_HEIGHT);
        }

        if position_ball.y < BALL_RAD{
        position_ball.y = BALL_RAD;
        direction_ball.y *= -1.0;
        speed_ball *= 1.01;
        cap_speed(&mut speed_ball, MAX_SPEED);
        }

        if position_ball.y + BALL_RAD > SCREEN_HEIGHT {
        position_ball.y = SCREEN_HEIGHT - BALL_RAD;
        direction_ball.y *= -1.0;
        speed_ball *= 1.01;
        cap_speed(&mut speed_ball, MAX_SPEED);
        }
//==================== end of ball logic (well i thought it was) --------

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        d.draw_text(&score_right.to_string(), SCREEN_WIDTH as i32 - 100, 50, 100, Color::WHITE);
        d.draw_text(&score_left.to_string(), 50, 50, 100, Color::WHITE);

        let player_1 = Rectangle::new(position_pdl_right.x, position_pdl_right.y, RECT_WIDTH, RECT_HEIGHT);
        let player_2 = Rectangle::new(position_pdl_left.x, position_pdl_left.y, RECT_WIDTH, RECT_HEIGHT);
        d.draw_rectangle_rec(player_1, Color::RED);
        d.draw_rectangle_rec(player_2, Color::BLUE);
        let ball_center = Vector2::new(position_ball.x, position_ball.y);
        let ball_radius : f32 = BALL_RAD;

        if player_1.check_collision_circle_rec(ball_center, ball_radius){
            direction_ball.x *= -1.0;
            speed_ball *= 1.01;
            cap_speed(&mut speed_ball, MAX_SPEED);
            position_ball.x = player_1.x - ball_radius;
        }

        if player_2.check_collision_circle_rec(ball_center, ball_radius){
            direction_ball.x *= -1.0;
            speed_ball *= 1.01;
            cap_speed(&mut speed_ball, MAX_SPEED);
            position_ball.x = player_2.x + player_2.width + ball_radius;
        }
        d.draw_fps(10, 10);
        d.draw_circle_v(ball_center, ball_radius, Color::new(255, 255,0, 255));

    }
}
