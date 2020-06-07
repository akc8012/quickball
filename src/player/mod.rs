use crate::physics::*;

mod input_component;
use input_component::InputComponent;

mod physics_component;
use physics_component::PhysicsComponent;

use quicksilver::{
	geom::{Circle, Rectangle, Vector},
	graphics::{Color, Image},
	Graphics, Input,
};

pub struct Player {
	input: InputComponent,
	physics: PhysicsComponent,

	pub pos: Vector,
	pub vel: Vector,
	pub radius: f32,
}

impl Player {
	pub fn new() -> Self {
		Player {
			input: InputComponent::new(),
			physics: PhysicsComponent::new(),
			pos: (300, 20).into(),
			vel: Vector::ZERO,
			radius: 16.,
		}
	}

	// TODO: somehow only pass Input to input component
	// TODO: pass in colliders via a World object
	pub fn update(&mut self, input: &Input, colliders: &[Box<dyn Collide>]) {
		self.physics.fall(&mut self.vel);

		if let Some(hit) = self
			.physics
			.grounded((&mut self.pos, &mut self.vel, self.radius), colliders)
		{
			if !self.physics.snap_to_ground(&mut self.pos, &mut self.vel, hit) {
				self.input.jump_if_pressed(&mut self.vel, input);
			}
		}
		self.input.set_jump_key_released(input);

		self.input.roll(&mut self.vel, input);
		self.pos = self.physics.update_position(self.pos, self.vel);
	}

	pub fn reset(&mut self) {
		self.pos = (300, 20).into();
		self.vel = Vector::ZERO;
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
