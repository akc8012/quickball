use super::colliders::Colliders;
use quicksilver::geom::Vector;

pub struct Ray {
	pub origin: Vector,
	pub direction: Vector,
	pub max_distance: f32,
}

#[derive(Clone)]
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

		let mut highest: Option<Hit> = None;

		for collider in colliders.get().filter(|c| c.y() >= self.origin.y) {
			let overflow: Vector = (self.origin + distance) - collider.pos();
			let within_x: bool = ray_x >= collider.top_left().x && ray_x <= collider.top_right().x;

			if overflow.y >= 0. && within_x {
				if let Some(h) = &highest {
					if collider.y() >= h.point.y {
						continue;
					}
				}

				highest = Some(Hit {
					point: (ray_x, collider.y()).into(),
					distance: distance - overflow,
				});
			}
		}

		highest
	}
}
