use raylib::prelude::*;
use raylib::consts::DEG2RAD;
use crate::config::{SCREEN_WIDTH, SCREEN_HEIGHT, BALL_RADIUS, BALL_SPEED, MAX_SPEED};


pub struct Ball {
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
        while angle % 90 == 0 {
            angle = rl.get_random_value(15..165);
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
