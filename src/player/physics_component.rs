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

	pub fn grounded(&self, bounds: &dyn Bounds, vel: &Vector, colliders: &[Box<dyn Bounds>]) -> Option<Hit> {
		let direction = (0., 1.).into();
		let distance = bounds.radius() + vel.y;

		let rays = vec![
			Ray::new(bounds.pos(), direction, Some(distance)),
			Ray::new(
				bounds.pos() - (bounds.radius() * 0.85, 0).into(),
				direction,
				Some(distance - 3.),
			),
			Ray::new(
				bounds.pos() + (bounds.radius() * 0.85, 0).into(),
				direction,
				Some(distance - 3.),
			),
		];

		for ray in rays {
			if let Some(hit) = ray.cast(colliders) {
				return Some(hit);
			}
		}

		None
	}

	pub fn snap_to_ground(&self, bounds: &mut dyn Bounds, vel: &mut Vector, hit: Hit) -> bool {
		let last_y = bounds.y();
		bounds.set_y(hit.point.y - hit.distance.y + vel.y);

		vel.y = 0.;
		bounds.y() > last_y
	}

	pub fn update_position(&self, bounds: &mut dyn Bounds, vel: &Vector) {
		bounds.set_pos(bounds.pos() + *vel);
	}
}
