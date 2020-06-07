pub mod rectangle_bounds;
use rectangle_bounds::RectangleBounds;

pub mod point_bounds;
use point_bounds::PointBounds;

pub mod circle_bounds;

use super::Bounds;
use quicksilver::{geom::Vector, Graphics, Input};

pub struct Colliders {
	colliders: Vec<Box<dyn Bounds>>,
}

impl Colliders {
	pub fn new(size: Vector) -> Self {
		Colliders {
			colliders: Self::create_colliders(size),
		}
	}

	pub fn get(&self) -> &Vec<Box<dyn Bounds>> {
		&self.colliders
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
		for collider in &self.colliders {
			collider.draw(gfx);
		}
	}
}
