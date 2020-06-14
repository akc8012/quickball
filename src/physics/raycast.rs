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

const DEFAULT_MAX_DISTANCE: f32 = 100.;

impl Ray {
	pub fn new(origin: Vector, direction: Vector, max_distance: Option<f32>) -> Self {
		Self {
			origin,
			direction,
			max_distance: match max_distance {
				Some(d) => d,
				None => DEFAULT_MAX_DISTANCE,
			},
		}
	}

	pub fn cast(&self, colliders: &Colliders) -> Option<Hit> {
		let distance: Vector = self.direction * self.max_distance;
		let ray_x: f32 = self.origin.x.round();

		let mut highest: Option<Hit> = None;

		for collider in colliders.get().filter(|c| c.y() >= self.origin.y) {
			let overflow_y: f32 = ((self.origin + distance) - collider.pos()).y;
			let within_x: bool = ray_x >= collider.top_left().x && ray_x <= collider.top_right().x;

			if overflow_y >= 0. && within_x {
				if let Some(h) = &highest {
					if collider.y() >= h.point.y {
						continue;
					}
				}

				highest = Some(Hit {
					point: (ray_x, collider.y()).into(),
					distance: (distance.x, distance.y - overflow_y).into(),
				});
			}
		}

		highest
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::physics::colliders::{point_bounds::PointBounds, rectangle_bounds::RectangleBounds};
	use quicksilver::geom::Rectangle;

	#[test]
	fn create_ray() {
		let default_distance = Ray::new(Vector::ZERO, (0, 1).into(), None);
		let specific_distance = Ray::new((20, 30).into(), Vector::ONE, Some(6.));

		assert_eq!(default_distance.origin, Vector::ZERO);
		assert_eq!(default_distance.direction, (0, 1).into());
		assert_eq!(default_distance.max_distance, DEFAULT_MAX_DISTANCE);

		assert_eq!(specific_distance.origin, (20, 30).into());
		assert_eq!(specific_distance.direction, Vector::ONE);
		assert_eq!(specific_distance.max_distance, 6.);
	}

	#[test]
	fn cast_zero_colliders() {
		let ray = Ray::new(Vector::ZERO, (0, 1).into(), None);
		let colliders = Colliders::create(Vec::new(), false);

		let hit = ray.cast(&colliders);
		assert!(hit.is_none());
	}

	#[test]
	fn cast_one_collider_no_hit() {
		let ray = Ray::new(Vector::ZERO, (0, 1).into(), Some(6.));

		let floor_below = Rectangle::new((-20, 7.), (40, 5));
		let colliders = Colliders::create(vec![Box::new(RectangleBounds::from(floor_below))], false);

		let hit = ray.cast(&colliders);
		assert!(hit.is_none());
	}

	#[test]
	fn cast_one_rect_collider_hit() {
		let ray = Ray::new((5, -1).into(), (0, 1).into(), Some(6.));

		let floor = Rectangle::new((-20.1, 3.3), (40.5, 5.1));
		let colliders = Colliders::create(vec![Box::new(RectangleBounds::from(floor))], false);

		let hit = ray.cast(&colliders).unwrap();
		assert_eq!(hit.point, (ray.origin.x, floor.pos.y).into());
		assert_eq!(hit.distance, hit.point - ray.origin);
	}

	#[test]
	fn cast_one_point_collider_hit() {
		let ray = Ray::new((2.2, -1).into(), (0, 1).into(), Some(8.));

		let point = (2, 5);
		let colliders = Colliders::create(vec![Box::new(PointBounds::new(point.0, point.1))], false);

		let hit = ray.cast(&colliders).unwrap();
		assert_eq!(hit.point, (2., 5.).into());
		assert_eq!(hit.distance, (0., 6.).into());
	}
}
