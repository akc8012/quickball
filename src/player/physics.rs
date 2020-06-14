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

	pub fn build_rays(&self, bounds: &dyn Bounds, vel: &Vector) -> Vec<Ray> {
		let direction = (0., 1.).into();
		let max_distance = bounds.radius() + vel.y;

		vec![Ray::new(bounds.pos(), direction, Some(max_distance))]
	}

	pub fn grounded(&self, rays: &[Ray], colliders: &Colliders) -> Option<Hit> {
		for ray in rays {
			if let Some(hit) = ray.cast(colliders) {
				return Some(hit);
			}
		}
		None
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
	fn build_rays() {
		let physics = PhysicsComponent::new();
		let bounds = CircleBounds::new((0, -1).into(), 3.);
		let vel = Vector::new(0, 2.);

		let rays = physics.build_rays(&bounds, &vel);
		assert_eq!(rays[0].origin, bounds.pos());
		assert_eq!(rays[0].direction, (0, 1).into());
		assert_eq!(rays[0].max_distance, bounds.radius() + vel.y);
	}

	#[test]
	fn grounded() {
		let physics = PhysicsComponent::new();
		let rays = vec![Ray {
			origin: (0, -1).into(),
			direction: (0, 1).into(),
			max_distance: 600.,
		}];

		let floor = Rectangle::new((-5, 3), (5, 10));
		let colliders = Colliders::create(vec![Box::new(RectangleBounds::from(floor))], false);

		let hit = physics.grounded(&rays, &colliders).unwrap();
		assert_eq!(hit.point, (rays[0].origin.x, floor.y()).into());

		assert_eq!(hit.distance, (0, 4).into());
		assert_eq!(hit.distance.y, floor.pos.y - rays[0].origin.y);
	}

	#[test]
	fn snap_to_ground() {
		let physics = PhysicsComponent::new();

		let mut bounds = CircleBounds::new((0, -1).into(), 3.);
		let mut vel = Vector::new(0, 12.);
		let hit = Hit {
			point: (0, 3).into(),
			distance: (0, 4).into(),
		};

		let snapped = physics.snap_to_ground(&mut bounds, &mut vel, hit);

		assert!(snapped);
		assert_eq!(bounds.pos(), (0, 0).into());
		assert_eq!(vel, (0, 0).into());
	}
}
