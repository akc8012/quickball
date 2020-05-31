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

pub fn cast(ray: Ray, colliders: &'_ [Collider]) -> Option<&'_ Collider> {
	for collider in colliders {
		let exceeding_y = (ray.origin + (ray.direction * ray.max_distance)).y >= collider.y();
		let within_x = ray.origin.x > collider.top_left().x && ray.origin.x < collider.top_right().x;

		if exceeding_y && within_x {
			return Some(collider);
		}
	}
	None
}
