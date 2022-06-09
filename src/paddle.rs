
use raylib::{prelude::{RaylibDrawHandle}, consts::KeyboardKey, ffi::Rectangle};

#[derive(Copy, Clone)]
pub struct Paddle {
	x: f32,
	y: f32,
	height: f32,
	width: f32,
	speed_y: f32,
	screen_height: i32,
}

impl Paddle {
	pub fn new(x: f32, y: f32, height: f32, width: f32, speed_y: f32, screen_height: i32) -> Paddle {
		Paddle {
			x: x,
			y: y,
			height: height,
			width: width,
			speed_y: speed_y,
			screen_height: screen_height,
		}
	}
	pub fn bottom_bound(&self) -> f32 {
		self.y + self.height
	}
	pub fn top_bound(&self) -> f32 {
		self.y
	}
	pub fn left_bound(&self) -> f32{
		self.x
	}
	pub fn right_bound(&self) -> f32{
		self.x + self.width
	}

	pub fn rect(&self) -> Rectangle {
		Rectangle {
			x: self.x,
			y: self.y,
			height: self.height,
			width: self.width,
		}
	}

	pub fn tick(&mut self, dt: f32, d: &RaylibDrawHandle) {
		println!("ticking");
		println!("top pos:    {}", self.top_bound());
		println!("bottom pos: {}", self.bottom_bound());


		// Movement
		if d.is_key_down(KeyboardKey::KEY_UP) {
			println!("up");
			self.y -= self.speed_y * dt;
			if self.top_bound() <= 0.0 {
				self.y = 0.0;
			}
			if self.bottom_bound() >= d.get_screen_height() as f32 {
				self.y = d.get_screen_height() as f32;
			}
		}
		if d.is_key_down(KeyboardKey::KEY_DOWN) {
			println!("down");
			self.y += self.speed_y * dt;
			if self.top_bound() <= 0.0 {
				self.y = 0.0;
			}
			if self.bottom_bound() >= self.screen_height as f32{
				self.y = self.screen_height as f32 - self.height;
			}
		}
		println!("paddle y: {}", self.y);
	}
}