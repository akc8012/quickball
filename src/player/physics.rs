use crate::{
	components::draw::{circle::DrawCircleComponent, DrawComponent},
	physics::{
		colliders::{circle_bounds::CircleBounds, Colliders},
		raycast::*,
		Bounds,
	},
};
use quicksilver::{geom::Vector, graphics::Color, Graphics};

pub struct PhysicsComponent {
	last_hit: Option<Hit>,
}
const GRAVITY: f32 = 2.;

impl PhysicsComponent {
	pub fn new() -> Self {
		PhysicsComponent { last_hit: None }
	}

	pub fn fall(&mut self, vel: &mut Vector) {
		vel.y += GRAVITY;

		// TODO: Should go on a more generic `update` style method?
		self.last_hit = None;
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
			// 	Some(max_distance - 3.),
			// ),
			// Ray::new(
			// 	bounds.pos() + (bounds.radius() * 0.85, 0).into(),
			// 	direction,
			// 	Some(max_distance - 3.),
			// ),
		]
	}

	pub fn snap_to_ground(&mut self, bounds: &mut dyn Bounds, vel: &mut Vector, hit: Hit) -> bool {
		vel.y = 0.;
		self.last_hit = Some(hit.clone());

		let last_y = bounds.y();
		let new_y = bounds.set_y(bounds.y() + hit.distance.y - bounds.radius());
		new_y > last_y
	}

	pub fn update_position(&self, bounds: &mut dyn Bounds, vel: &Vector) {
		bounds.set_pos(bounds.pos() + *vel);
	}

	pub fn draw(&self, gfx: &mut Graphics) {
		if let Some(hit) = &self.last_hit {
			let hit_circle = DrawCircleComponent::new(Color::RED);
			hit_circle.draw(gfx, Some(&CircleBounds::new(hit.point, 2.5)));
		}
	}
}
