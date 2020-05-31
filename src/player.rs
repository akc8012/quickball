use quicksilver::{
	geom::{Circle, Vector},
	graphics::Color,
	input::Key,
	Graphics, Input,
};

use crate::{
	collider::Collider,
	raycast::{self, Ray},
};

pub struct Player {
	pos: Vector,
	vel: Vector,
	radius: f32,
	jump_key_released: bool,
}

impl Player {
	pub fn new() -> Self {
		Player {
			pos: (300, 20).into(),
			vel: Vector::ZERO,
			radius: 16.0,
			jump_key_released: true,
		}
	}

	pub fn reset(&mut self) {
		self.pos = (300, 20).into();
		self.vel = Vector::ZERO;
	}

	pub fn update(&mut self, input: &Input, colliders: &Vec<Collider>, _size: Vector) {
		self.fall();

		if self.grounded(colliders) {
			// TODO: Can't just rely on ground being the first Collider!!
			let snapped_this_frame = self.snap_to_ground(&colliders[0]);
			if !snapped_this_frame && self.can_jump(input) {
				self.jump();
			}
		}
		self.set_jump_key_released(input);

		self.roll(input);
		self.update_position();
	}

	fn fall(&mut self) {
		const GRAVITY: f32 = 2.0;
		self.vel.y += GRAVITY;
	}

	fn grounded(&self, colliders: &Vec<Collider>) -> bool {
		let ray = Ray::new(self.pos, (0.0, 1.0).into(), Some(self.radius + self.vel.y));
		raycast::cast(ray, colliders)
	}

	fn snap_to_ground(&mut self, ground: &Collider) -> bool {
		self.vel.y = 0.0;

		let last_y = self.pos.y;
		self.pos.y = ground.bounds().y() - self.radius;
		self.pos.y > last_y
	}

	fn can_jump(&self, input: &Input) -> bool {
		(input.key_down(Key::W) || input.key_down(Key::Up)) && self.jump_key_released
	}

	fn jump(&mut self) {
		const JUMP_HEIGHT: f32 = 20.0;

		self.vel.y -= JUMP_HEIGHT;
		self.jump_key_released = false;
	}

	fn set_jump_key_released(&mut self, input: &Input) {
		// TODO: Base this off of framestamp
		if !input.key_down(Key::W) {
			self.jump_key_released = true;
		}
	}

	fn roll(&mut self, input: &Input) {
		const ROLL_SPEED: f32 = 4.0;
		self.vel.x = 0.0;

		if input.key_down(Key::A) || input.key_down(Key::Left) {
			self.vel.x -= ROLL_SPEED;
		}
		if input.key_down(Key::D) || input.key_down(Key::Right) {
			self.vel.x += ROLL_SPEED;
		}
	}

	fn update_position(&mut self) {
		self.pos += self.vel;
	}

	pub fn draw(&self, gfx: &mut Graphics) {
		gfx.fill_circle(&Circle::new(self.pos, self.radius), Color::from_hex("4f30d9"));
	}
}
