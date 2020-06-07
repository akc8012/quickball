use quicksilver::{
	geom::{Circle, Rectangle, Vector},
	graphics::{Color, Image},
	Graphics, Input,
};

use crate::{
	collider::Collide,
	player_input::PlayerInput,
	raycast::{self, Hit, Ray},
};

pub struct Player {
	input: PlayerInput,

	pub pos: Vector,
	pub vel: Vector,
	pub radius: f32,
}

impl Player {
	pub fn new() -> Self {
		Player {
			input: PlayerInput::new(),
			pos: (300, 20).into(),
			vel: Vector::ZERO,
			radius: 16.,
		}
	}

	pub fn reset(&mut self) {
		self.pos = (300, 20).into();
		self.vel = Vector::ZERO;
	}

	// TODO: somehow only pass Input to input component
	pub fn update(&mut self, input: &Input, colliders: &[Box<dyn Collide>], _size: Vector) {
		self.fall();

		if let Some(hit) = self.grounded(colliders) {
			if !self.snap_to_ground(hit) {
				self.input = self.input.jump_if_pressed(self, input);
			}
		}
		self.input.set_jump_key_released(input);

		self.input.roll(self, input);
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
