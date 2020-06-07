use crate::physics::Collide;
use quicksilver::geom::Vector;

pub struct Ray {
	pub origin: Vector,
	pub direction: Vector,
	pub max_distance: f32,
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

pub struct Hit {
	pub point: Vector,
	pub distance: Vector,
}

pub fn cast(ray: Ray, colliders: &[Box<dyn Collide>]) -> Option<Hit> {
	for collider in colliders {
		let distance: Vector = ray.direction * ray.max_distance;
		let exceeding_y: bool = (ray.origin + distance).y >= collider.y();

		let ray_x: f32 = ray.origin.x.round();
		let within_x: bool = ray_x >= collider.top_left().x && ray_x <= collider.top_right().x;

		if exceeding_y && within_x {
			let point: Vector = (ray.origin.x, collider.y()).into();
			return Some(Hit { point, distance });
		}
	}
	None
}
