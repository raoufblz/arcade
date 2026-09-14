use crate::config::{BALL_RAD, BALL_SPEED, MAX_SPEED, SCREEN_HEIGHT, SCREEN_WIDTH};
use raylib::consts::DEG2RAD;
use raylib::prelude::*;

pub struct Ball {
    pub position: Vector2,
    pub direction: Vector2,
    pub speed: f32,
    pub radius: f32,
}

impl Ball {
    pub fn new(rl: &mut RaylibHandle,position: Vector2) -> Self {
        let mut ball =Self {
            position,
            direction: Vector2::zero(),
            speed: BALL_SPEED,
            radius: BALL_RAD,
        };
        ball.randomize_direction(rl);
        ball
    }

    pub fn update(&mut self, delta: f32) {
        self.position.x += self.direction.x * self.speed * delta;
        self.position.y += self.direction.y * self.speed * delta;
    }

    pub fn reset(&mut self, rl: &mut RaylibHandle) {
        self.position = Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0);
        self.speed = BALL_SPEED;
        self.randomize_direction(rl);
    }

    pub fn cap_speed(&mut self) {
        if self.speed > MAX_SPEED {
            self.speed = MAX_SPEED;
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle_v(self.position, self.radius, Color::new(255, 255, 0, 255));
    }

    fn randomize_direction(&mut self, rl: &mut RaylibHandle) {
		let mut angle = 0;
		while angle % 90 == 0 || (angle > 75 && angle < 105) || (angle > 255 && angle < 285) {
		angle = rl.get_random_value(1..360);
		}
		let radians: f32 = angle as f32 * DEG2RAD as f32;
		let direction = Vector2::new(radians.cos(), radians.sin());
		self.direction = direction;
    }
}
