use crate::{
	components::draw::{circle::DrawCircleComponent, rectangle::DrawRectangleComponent, DrawComponent},
	physics::{
		colliders::{circle_bounds::CircleBounds, rectangle_bounds::RectangleBounds, Colliders},
		raycast::*,
		Bounds,
	},
};
use quicksilver::{geom::Vector, graphics::Color, Graphics};

pub struct PhysicsComponent {
	last_hit: Option<Hit>,
	rays: Vec<Ray>,
}
const GRAVITY: f32 = 2.;

impl PhysicsComponent {
	pub fn new() -> Self {
		PhysicsComponent {
			last_hit: None,
			rays: Vec::new(),
		}
	}

	pub fn fall(&mut self, vel: &mut Vector) {
		vel.y += GRAVITY;

		// TODO: Should go on a more generic `update` style method?
		self.last_hit = None;
	}

	pub fn grounded(&mut self, bounds: &dyn Bounds, vel: &Vector, colliders: &Colliders) -> Option<Hit> {
		self.rays = self.build_rays(bounds, vel);

		for ray in &self.rays {
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
			hit_circle.draw(gfx, Some(&CircleBounds::new(hit.point, 3.)));
		}

		for ray in &self.rays {
			let ray_line = DrawRectangleComponent::new(Color::RED);

			let line_width = 1.5;
			let bounds = RectangleBounds::new(
				(ray.origin.x - (line_width / 2.), ray.origin.y),
				(line_width, ray.max_distance),
			);

			ray_line.draw(gfx, Some(&bounds));
		}
	}
}
