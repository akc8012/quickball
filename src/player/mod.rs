use crate::{
	components::draw_component::DrawComponent,
	physics::{colliders::Colliders, Bounds},
};

mod input_component;
use input_component::InputComponent;

mod physics_component;
use physics_component::PhysicsComponent;

mod draw_image_component;
use draw_image_component::DrawImageComponent;

use quicksilver::{geom::Vector, graphics::Image, Graphics, Input};

pub struct Player {
	input: InputComponent,
	physics: PhysicsComponent,
	draw: DrawImageComponent,

	bounds: Box<dyn Bounds>,
	vel: Vector,
}

impl Player {
	pub fn new(bounds: Box<dyn Bounds>, image: Option<Image>) -> Self {
		Player {
			input: InputComponent::new(),
			physics: PhysicsComponent::new(),
			draw: DrawImageComponent::new(image),

			bounds,
			vel: Vector::ZERO,
		}
	}

	// TODO: somehow only pass Input to input component
	// TODO: pass in colliders via a World object
	pub fn update(&mut self, input_: &Input, colliders: &Colliders) {
		let (physics, input) = (&self.physics, &mut self.input);

		physics.fall(&mut self.vel);

		if let Some(hit) = physics.grounded(self.bounds.as_ref(), &self.vel, colliders) {
			if !physics.snap_to_ground(self.bounds.as_mut(), &mut self.vel, hit) {
				input.jump_if_pressed(&mut self.vel, input_);
			}
		}
		input.set_jump_key_released(input_);

		input.roll(&mut self.vel, input_);
		physics.update_position(self.bounds.as_mut(), &self.vel);
	}

	pub fn reset(&mut self) {
		self.bounds.set_pos((300, 20).into());
		self.vel = Vector::ZERO;
	}

	pub fn draw(&self, gfx: &mut Graphics) {
		self.draw.draw(gfx, Some(self.bounds.as_ref()));
	}
}
