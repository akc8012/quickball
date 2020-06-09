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

		let mut highest = Hit {
			point: Vector::new(10000., 10000.),
			distance: Vector::ZERO,
		};

		for collider in colliders.get().filter(|c| c.y() >= self.origin.y) {
			let overflow: Vector = (self.origin + distance) - collider.pos();
			let within_x: bool = ray_x >= collider.top_left().x && ray_x <= collider.top_right().x;

			if overflow.y >= 0. && within_x {
				let point: Vector = (ray_x, collider.y()).into();
				if point.y < highest.point.y {
					highest = Hit {
						point,
						distance: distance - overflow,
					};
				}
			}
		}

		// TODO: this check is garbage
		if highest.distance == Vector::ZERO {
			None
		} else {
			Some(highest)
		}
	}
}
