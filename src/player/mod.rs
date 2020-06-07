use crate::physics::*;

mod input_component;
use input_component::InputComponent;

mod physics_component;
use physics_component::PhysicsComponent;

mod draw_component;
use draw_component::DrawComponent;

use quicksilver::{geom::Vector, graphics::Image, Graphics, Input};

pub struct Player {
	input: InputComponent,
	physics: PhysicsComponent,
	draw: DrawComponent,

	bounds: CircleBounds,
	vel: Vector,
}

impl Player {
	pub fn new() -> Self {
		Player {
			input: InputComponent::new(),
			physics: PhysicsComponent::new(),
			draw: DrawComponent::new(),

			bounds: CircleBounds::new((300, 20).into(), 16.),
			vel: Vector::ZERO,
		}
	}

	// TODO: somehow only pass Input to input component
	// TODO: pass in colliders via a World object
	pub fn update(&mut self, input_: &Input, colliders: &[Box<dyn Bounds>]) {
		let (physics, input) = (&self.physics, &mut self.input);

		physics.fall(&mut self.vel);

		if let Some(hit) = physics.grounded(&self.bounds, &self.vel, colliders) {
			if !physics.snap_to_ground(&mut self.bounds, &mut self.vel, hit) {
				input.jump_if_pressed(&mut self.vel, input_);
			}
		}
		input.set_jump_key_released(input_);

		input.roll(&mut self.vel, input_);
		physics.update_position(&mut self.bounds, &self.vel);
	}

	pub fn reset(&mut self) {
		self.bounds.pos = (300, 20).into();
		self.vel = Vector::ZERO;
	}

	pub fn draw(&self, image: &Option<Image>, gfx: &mut Graphics) {
		self.draw.draw(&self.bounds, image, gfx);
	}
}
