use crate::config::{PADDLE_HEIGHT, PADDLE_WIDTH, PADDLE_SPEED, SCREEN_HEIGHT};
use raylib::prelude::*;

pub struct Paddle {
    pub position: Vector2,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
}

impl Paddle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vector2::new(x, y),
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
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
