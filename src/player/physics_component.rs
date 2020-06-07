use crate::physics::{raycast::*, Bounds};
use quicksilver::geom::Vector;

pub struct PhysicsComponent;
const GRAVITY: f32 = 2.;

impl PhysicsComponent {
	pub fn new() -> Self {
		PhysicsComponent {}
	}

	pub fn fall(&self, vel: &mut Vector) {
		vel.y += GRAVITY;
	}

	pub fn grounded(
		&self,
		(pos, vel, radius): (&mut Vector, &mut Vector, f32),
		colliders: &[Box<dyn Bounds>],
	) -> Option<Hit> {
		let direction = (0., 1.).into();
		let distance = radius + vel.y;

		let rays = vec![
			Ray::new(*pos, direction, Some(distance)),
			Ray::new(*pos - (radius * 0.85, 0).into(), direction, Some(distance - 3.)),
			Ray::new(*pos + (radius * 0.85, 0).into(), direction, Some(distance - 3.)),
		];

		for ray in rays {
			if let Some(hit) = ray.cast(colliders) {
				return Some(hit);
			}
		}

		None
	}

	pub fn snap_to_ground(&self, pos: &mut Vector, vel: &mut Vector, hit: Hit) -> bool {
		let last_y = pos.y;
		pos.y = hit.point.y - hit.distance.y + vel.y;

		vel.y = 0.;
		pos.y > last_y
	}

	pub fn update_position(&self, pos: &mut Vector, vel: &Vector) {
		*pos += *vel;
	}
}
