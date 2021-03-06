pub mod rectangle_bounds;
use rectangle_bounds::RectangleBounds;

pub mod point_bounds;
use point_bounds::PointBounds;

pub mod circle_bounds;

use super::Bounds;
use quicksilver::{
	geom::{Rectangle, Vector},
	graphics::Color,
	Graphics, Input,
};
use std::slice::Iter;

pub struct Colliders {
	colliders: Vec<Box<dyn Bounds>>,
	draw_colliders: bool,
}

impl Colliders {
	pub fn new(size: Vector, draw_colliders: bool) -> Self {
		Colliders {
			colliders: Self::create_colliders(size),
			draw_colliders,
		}
	}

	pub fn get(&self) -> Iter<Box<dyn Bounds>> {
		self.colliders.iter()
	}

	fn create_colliders(size: Vector) -> Vec<Box<dyn Bounds>> {
		let mut colliders: Vec<Box<dyn Bounds>> = Vec::new();

		// ground
		colliders.push(Box::new(RectangleBounds::new(
			(0.0, size.y - 20.0),
			(size.x, 32.0),
		)));

		// platform
		colliders.push(Box::new(RectangleBounds::new((525, 400), (128, 10))));

		// points
		for x in 0..300 {
			for y in 380..383 {
				colliders.push(Box::new(PointBounds::new((x, y).into())));
			}
		}

		colliders
	}

	pub fn update(&mut self, input: &Input) {
		if input.mouse().left() {
			self.colliders
				.push(Box::new(PointBounds::new(input.mouse().location())));

			for x in -5..5 {
				for y in -5..5 {
					self.colliders.push(Box::new(PointBounds::new(
						input.mouse().location() + (x, y).into(),
					)));
				}
			}
		}
	}

	pub fn draw(&self, gfx: &mut Graphics) {
		if !self.draw_colliders {
			return;
		}

		for collider in &self.colliders {
			if collider.should_draw() {
				gfx.fill_rect(
					&Rectangle::new(collider.pos(), (collider.width(), collider.height())),
					Color::GREEN,
				);
			}
		}
	}
}
