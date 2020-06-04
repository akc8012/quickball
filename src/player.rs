use quicksilver::{
	geom::{Circle, Rectangle, Vector},
	graphics::{Color, Image},
	input::Key,
	Graphics, Input,
};

use crate::{
	collider::Collide,
	raycast::{self, Hit, Ray},
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
			radius: 16.,
			jump_key_released: true,
		}
	}

	pub fn reset(&mut self) {
		self.pos = (300, 20).into();
		self.vel = Vector::ZERO;
	}

	pub fn update(&mut self, input: &Input, colliders: &[Box<dyn Collide>], _size: Vector) {
		self.fall();

		if let Some(hit) = self.grounded(colliders) {
			let snapped_this_frame = self.snap_to_ground(hit);
			if !snapped_this_frame && self.can_jump(input) {
				self.jump();
			}
		}
		self.set_jump_key_released(input);

		self.roll(input);
		self.update_position();
	}

	fn fall(&mut self) {
		const GRAVITY: f32 = 2.;
		self.vel.y += GRAVITY;
	}

	fn grounded(&self, colliders: &[Box<dyn Collide>]) -> Option<Hit> {
		let direction = (0., 1.).into();
		let distance = self.radius + self.vel.y;

		let rays = vec![
			Ray::new(self.pos, direction, Some(distance)),
			Ray::new(
				self.pos - (self.radius * 0.85, 0).into(),
				direction,
				Some(distance - 3.),
			),
			Ray::new(
				self.pos + (self.radius * 0.85, 0).into(),
				direction,
				Some(distance - 3.),
			),
		];

		for ray in rays {
			if let Some(hit) = raycast::cast(ray, colliders) {
				return Some(hit);
			}
		}

		None
	}

	fn snap_to_ground(&mut self, hit: Hit) -> bool {
		let last_y = self.pos.y;
		self.pos.y = hit.point.y - hit.distance.y + self.vel.y;

		self.vel.y = 0.;
		self.pos.y > last_y
	}

	fn can_jump(&self, input: &Input) -> bool {
		(input.key_down(Key::W) || input.key_down(Key::Up)) && self.jump_key_released
	}

	fn jump(&mut self) {
		const JUMP_HEIGHT: f32 = 20.;

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
		const ROLL_SPEED: f32 = 4.;
		self.vel.x = 0.;

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

	pub fn draw(&self, image: &Option<Image>, gfx: &mut Graphics) {
		if let Some(image) = image {
			gfx.draw_image(
				image,
				Rectangle::new(self.pos - (Vector::ONE * self.radius), image.size()),
			);
		} else {
			gfx.fill_circle(&Circle::new(self.pos, self.radius), Color::from_hex("4f30d9"));
		}
	}
}
