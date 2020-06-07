use crate::physics::{raycast::*, Collide};

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
		colliders: &[Box<dyn Collide>],
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
}
