use super::colliders::Colliders;
use quicksilver::geom::Vector;

pub struct Ray {
	pub origin: Vector,
	pub direction: Vector,
	pub max_distance: f32,
}

pub struct Hit {
	pub point: Vector,
	pub distance: Vector,
}

impl Ray {
	pub fn new(origin: Vector, direction: Vector, max_distance: Option<f32>) -> Self {
		Self {
			origin,
			direction,
			max_distance: match max_distance {
				Some(d) => d,
				None => 100.0,
			},
		}
	}

	pub fn cast(&self, colliders: &Colliders) -> Option<Hit> {
		let distance: Vector = self.direction * self.max_distance;
		let ray_x: f32 = self.origin.x.round();

		for collider in colliders.get() {
			let overflow: Vector = (self.origin + distance) - collider.pos();
			let within_x: bool = ray_x >= collider.top_left().x && ray_x <= collider.top_right().x;

			if overflow.y >= 0. && within_x {
				let point: Vector = (ray_x, collider.y()).into();
				return Some(Hit {
					point,
					distance: distance - overflow,
				});
			}
		}
		None
	}
}
