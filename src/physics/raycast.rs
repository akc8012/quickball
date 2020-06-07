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
		for collider in colliders.get() {
			let distance: Vector = self.direction * self.max_distance;
			let exceeding_y: bool = (self.origin + distance).y >= collider.y();

			let ray_x: f32 = self.origin.x.round();
			let within_x: bool = ray_x >= collider.top_left().x && ray_x <= collider.top_right().x;

			if exceeding_y && within_x {
				let point: Vector = (self.origin.x, collider.y()).into();
				return Some(Hit { point, distance });
			}
		}
		None
	}
}
