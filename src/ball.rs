use rand::Rng;
use raylib::ffi::Rectangle;
use crate::paddle::Paddle;

pub struct Ball {
	x: f32,
	y: f32,
	speed_x: f32,
	speed_y: f32,
	size: f32,
	screen_width: f32,
	screen_height: f32,
	score: i32,
}

impl Ball {
	pub fn new(x: f32, y: f32, speed_x: f32, speed_y: f32, size: f32, screen_width: f32, screen_height: f32) -> Ball {
		Ball {
			x: x,
			y: y,
			speed_x: speed_x,
			speed_y: speed_y,
			size: size,
			screen_width: screen_width,
			screen_height: screen_height,
			score: 0,
		}
	}


	

	fn top_bound(&self) -> f32 {
		self.y
	}
	fn bottom_bound(&self) -> f32 {
		self.y + self.size
	}
	fn left_bound(&self) -> f32 {
		self.x
	}
	fn right_bound(&self) -> f32 {
		self.x + self.size
	}
	pub fn get_score(&self) -> i32{
		self.score
	}

	pub fn rect(&self) -> Rectangle {
		Rectangle {
			x: self.x,
			y: self.y,
			height: self.size,
			width: self.size,
		}
	}

	pub fn score(&mut self) {
		self.score += 1;
		self.x = self.screen_width / 2.0;
		self.y = self.screen_height / 2.0;
		// self.speed_x = 0.0;
		// self.speed_y = 0.0;
	}

	fn bounce(&mut self) {
		let mut rng = rand::thread_rng();
		self.speed_x = -self.speed_x;
		if self.speed_y > 0.0 {
			self.speed_y = rng.gen_range(0.0..500.0)
		} else {
			self.speed_y = rng.gen_range(-500.0..0.0)
		}
	}

	pub fn tick(&mut self, dt: f32, paddle : Paddle) {

		if self.left_bound() <= paddle.right_bound() &&
		self.right_bound() >= paddle.left_bound() &&
		self.bottom_bound() >= paddle.top_bound() &&
		self.top_bound() <= paddle.bottom_bound() {
			self.bounce();
		}
		if self.top_bound() <= 0.0 {
			self.speed_y = -self.speed_y;
		}
		if self.bottom_bound() >= self.screen_height {
			self.speed_y = -self.speed_y;
		}
		if self.right_bound() >= self.screen_width {
			self.score += 1;
			self.bounce();
		}
		if self.left_bound() <= 0.0 {
			self.score();
		}


		self.x += self.speed_x * dt;
		self.y += self.speed_y * dt;
	}
}