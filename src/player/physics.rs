use crate::physics::{colliders::Colliders, raycast::*, Bounds};
use quicksilver::{geom::Vector, Graphics};

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
		for ray in self.build_rays(bounds, vel) {
			if let Some(hit) = ray.cast(colliders) {
				return Some(hit);
			}
		}
		None
	}

	fn build_rays(&self, bounds: &dyn Bounds, vel: &Vector) -> Vec<Ray> {
		let direction = (0., 1.).into();
		let max_distance = bounds.radius() + vel.y;

		vec![Ray::new(bounds.pos(), direction, Some(max_distance))]
	}

	pub fn snap_to_ground(&self, bounds: &mut dyn Bounds, vel: &mut Vector, hit: Hit) -> bool {
		vel.y = 0.;

		let last_y = bounds.y();
		let new_y = bounds.set_y(bounds.y() + hit.distance.y - bounds.radius());
		new_y > last_y
	}

	pub fn update_position(&self, bounds: &mut dyn Bounds, vel: &Vector) {
		bounds.set_pos(bounds.pos() + *vel);
	}

	pub fn draw(&self, _gfx: &mut Graphics) {
		// no-op
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::physics::colliders::{circle_bounds::CircleBounds, rectangle_bounds::RectangleBounds};
	use quicksilver::geom::Rectangle;

	#[test]
	fn fall() {
		let physics = PhysicsComponent::new();
		let mut vel = Vector::new(0, 3.);

		physics.fall(&mut vel);
		assert_eq!(vel, (0, 3. + GRAVITY).into());
	}

	#[test]
	fn grounded() {
		let physics = PhysicsComponent::new();
		let bounds = CircleBounds::new((0.1, 0.3).into(), 6.2);
		let vel = Vector::new(0, 6.);

		let floor = Rectangle::new((-10.2, 4.1), (20, 24));
		let colliders = Colliders::create(vec![Box::new(RectangleBounds::from(floor))], false);

		let hit = physics.grounded(&bounds, &vel, &colliders).unwrap();
		assert_eq!(hit.point, (bounds.x(), floor.y()).into());
	}
}
