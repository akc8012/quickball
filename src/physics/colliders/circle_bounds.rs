use crate::physics::Bounds;
use quicksilver::geom::Vector;

pub struct CircleBounds {
	pub pos: Vector,
	pub radius: f32,
}

impl CircleBounds {
	pub fn new(pos: Vector, radius: f32) -> Self {
		Self { pos, radius }
	}
}

impl Bounds for CircleBounds {
	fn x(&self) -> f32 {
		self.pos.x
	}

	fn y(&self) -> f32 {
		self.pos.y
	}

	fn pos(&self) -> Vector {
		self.pos
	}

	fn top_left(&self) -> Vector {
		todo!()
	}

	fn top_right(&self) -> Vector {
		todo!()
	}

	fn width(&self) -> f32 {
		self.radius * 2.
	}

	fn height(&self) -> f32 {
		self.width()
	}

	fn radius(&self) -> f32 {
		self.radius
	}

	fn set_pos(&mut self, pos: Vector) -> Vector {
		self.pos = pos;
		self.pos
	}

	fn set_x(&mut self, x: f32) -> f32 {
		self.pos.x = x;
		self.pos.x
	}

	fn set_y(&mut self, y: f32) -> f32 {
		self.pos.y = y;
		self.pos.y
	}

	fn should_draw(&self) -> bool {
		false
	}
}
