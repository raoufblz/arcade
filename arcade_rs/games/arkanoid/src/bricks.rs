use raylib::prelude::*;
use crate::config::{BRICK_WIDTH, BRICK_HEIGHT};


pub struct Brick {
    pub position: Vector2,
    pub width: f32,
    pub height: f32,
    pub broken: bool,
}

impl Brick {
    pub fn new(position: Vector2) -> Self {
        Self {
            position,
            width: BRICK_WIDTH,
            height: BRICK_HEIGHT,
            broken: false,
        }
    }

   	// the loop breaks them, bricks don t update

	pub fn is_broken(&self) -> bool {
		self.broken
	}

	pub fn do_break(&mut self) {
		self.broken = true;
	}

    pub fn get_rect(&self) -> Rectangle {
        Rectangle::new(self.position.x, self.position.y, self.width, self.height)
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, color: Color) {
        d.draw_rectangle_rec(self.get_rect(), color);
    }
}

