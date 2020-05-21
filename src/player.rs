use quicksilver::{
	geom::{Circle, Vector},
	graphics::Color,
	input::Key,
	Graphics, Input,
};

pub struct Player {
	pos: Vector,
	vel: Vector,
	radius: f32,
}

impl Player {
	pub fn new() -> Self {
		Player {
			pos: (300, 20).into(),
			vel: Vector::ZERO,
			radius: 16.0,
		}
	}

	pub fn reset(&mut self) {
		self.pos = (300, 20).into();
		self.vel = Vector::ZERO;
	}

	pub fn update(&mut self, input: &Input) {
		const ROLL_SPEED: f32 = 4.0;
		const GRAVITY: f32 = 2.0;
		const JUMP_HEIGHT: f32 = 20.0;

		self.vel.x = 0.0;
		self.vel.y += GRAVITY;

		if self.pos.y + self.radius >= 480.0 {
			self.vel.y = 0.0;
			self.pos.y = 480.0 - self.radius;

			if input.key_down(Key::W) {
				self.vel.y -= JUMP_HEIGHT;
			}
		}

		if input.key_down(Key::A) {
			self.vel.x -= ROLL_SPEED;
		}
		if input.key_down(Key::D) {
			self.vel.x += ROLL_SPEED;
		}

		self.pos += self.vel;
	}

	pub fn draw(&self, gfx: &mut Graphics) {
		gfx.fill_circle(&Circle::new(self.pos, self.radius), Color::from_hex("4f30d9"));
	}
}
