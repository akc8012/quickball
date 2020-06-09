use crate::physics::{colliders::Colliders, raycast::*, Bounds};
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

	pub fn grounded(&self, bounds: &dyn Bounds, vel: &Vector, colliders: &Colliders) -> Option<Hit> {
		let rays = self.build_rays(bounds, vel);

		for ray in rays {
			if let Some(hit) = ray.cast(colliders) {
				return Some(hit);
			}
		}
		None
	}

	fn build_rays(&self, bounds: &dyn Bounds, vel: &Vector) -> Vec<Ray> {
		let direction = (0., 1.).into();
		let max_distance = bounds.radius() + vel.y;

		vec![
			Ray::new(bounds.pos(), direction, Some(max_distance)),
			// Ray::new(
			// 	bounds.pos() - (bounds.radius() * 0.85, 0).into(),
			// 	direction,
			// 	Some(distance - 3.),
			// ),
			// Ray::new(
			// 	bounds.pos() + (bounds.radius() * 0.85, 0).into(),
			// 	direction,
			// 	Some(distance - 3.),
			// ),
		]
	}

	pub fn snap_to_ground(&self, bounds: &mut dyn Bounds, vel: &mut Vector, hit: Hit) -> bool {
		let last_y = bounds.y();
		bounds.set_y(bounds.y() + hit.distance.y - bounds.radius());

		vel.y = 0.;
		bounds.y() > last_y
	}

	pub fn update_position(&self, bounds: &mut dyn Bounds, vel: &Vector) {
		bounds.set_pos(bounds.pos() + *vel);
	}
}
