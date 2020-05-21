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
	jump_key_released: bool,
}

impl Player {
	pub fn new() -> Self {
		Player {
			pos: (300, 20).into(),
			vel: Vector::ZERO,
			radius: 16.0,
			jump_key_released: true,
		}
	}

	pub fn reset(&mut self) {
		self.pos = (300, 20).into();
		self.vel = Vector::ZERO;
	}

	pub fn update(&mut self, input: &Input) {
		self.fall();
		if self.grounded() {
			self.stick_to_ground();

			if self.can_jump(input) {
				self.jump();
			}
		}
		self.set_jump_key_released(input);

		self.roll(input);
		self.update_position();
	}

	fn fall(&mut self) {
		const GRAVITY: f32 = 2.0;
		self.vel.y += GRAVITY;
	}

	fn grounded(&self) -> bool {
		// TODO: Pass in resolution
		self.pos.y + self.radius >= 480.0
	}

	fn stick_to_ground(&mut self) {
		self.vel.y = 0.0;
		self.pos.y = 480.0 - self.radius;
	}

	fn can_jump(&self, input: &Input) -> bool {
		input.key_down(Key::W) && self.jump_key_released
	}

	fn jump(&mut self) {
		const JUMP_HEIGHT: f32 = 20.0;

		self.vel.y -= JUMP_HEIGHT;
		self.jump_key_released = false;
	}

	fn set_jump_key_released(&mut self, input: &Input) {
		if !input.key_down(Key::W) {
			self.jump_key_released = true;
		}
	}

	fn roll(&mut self, input: &Input) {
		const ROLL_SPEED: f32 = 4.0;
		self.vel.x = 0.0;

		if input.key_down(Key::A) {
			self.vel.x -= ROLL_SPEED;
		}
		if input.key_down(Key::D) {
			self.vel.x += ROLL_SPEED;
		}
	}

	fn update_position(&mut self) {
		self.pos += self.vel;
	}

	pub fn draw(&self, gfx: &mut Graphics) {
		gfx.fill_circle(&Circle::new(self.pos, self.radius), Color::from_hex("4f30d9"));
	}
}
