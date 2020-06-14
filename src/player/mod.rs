use crate::{
	components::draw::DrawComponent,
	physics::{colliders::Colliders, Bounds},
};

mod input;
use input::InputComponent;

mod physics;
use physics::PhysicsComponent;

use quicksilver::{geom::Vector, Graphics, Input};

pub struct Player {
	input: InputComponent,
	physics: PhysicsComponent,

	bounds: Box<dyn Bounds>,
	draw: Box<dyn DrawComponent>,

	vel: Vector,
}

impl Player {
	pub fn new(bounds: Box<dyn Bounds>, draw: Box<dyn DrawComponent>) -> Self {
		Player {
			input: InputComponent::new(),
			physics: PhysicsComponent::new(),
			bounds,
			draw,
			vel: Vector::ZERO,
		}
	}

	// TODO: somehow only pass Input to input component
	// TODO: pass in colliders via a World object
	pub fn update(&mut self, input_: &Input, colliders: &Colliders) {
		let (physics, input) = (&mut self.physics, &mut self.input);

		physics.fall(&mut self.vel);

		let rays = physics.build_rays(&*self.bounds, &self.vel);
		if let Some(hit) = physics.grounded(&rays, colliders) {
			if !physics.snap_to_ground(&mut *self.bounds, &mut self.vel, hit) {
				input.jump_if_pressed(&mut self.vel, input_);
			}
		}
		input.set_jump_key_released(input_);

		input.roll(&mut self.vel, input_);
		physics.update_position(&mut *self.bounds, &self.vel);
	}

	pub fn reset(&mut self) {
		self.bounds.set_pos((300, 20).into());
		self.vel = Vector::ZERO;
	}

	pub fn draw(&self, gfx: &mut Graphics, debug_draw: bool) {
		self.draw.draw(gfx, Some(&*self.bounds));

		if debug_draw {
			self.physics.draw(gfx);
		}
	}
}
