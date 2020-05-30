use crate::collider::Collider;
use quicksilver::geom::Vector;

pub struct Ray {
	origin: Vector,
	direction: Vector,
	max_distance: f32,
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
}

pub fn cast(ray: Ray, collider: Collider) -> bool {
	(ray.origin + (ray.direction * ray.max_distance)).y >= collider.bounds().pos.y
}
