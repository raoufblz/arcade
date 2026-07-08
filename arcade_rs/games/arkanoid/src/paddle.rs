use raylib::prelude::*;
use crate::config::{PADDLE_WIDTH, PADDLE_HEIGHT, PADDLE_SPEED};


pub struct Paddle {
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
