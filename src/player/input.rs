use quicksilver::{geom::Vector, input::Key, Input};

pub struct InputComponent {
	jump_key_released: bool,
}

const JUMP_HEIGHT: f32 = 20.;
const ROLL_SPEED: f32 = 4.2;

impl InputComponent {
	pub fn new() -> Self {
		InputComponent {
			jump_key_released: true,
		}
	}

	pub fn roll(&self, vel: &mut Vector, input: &Input) {
		vel.x = 0.;

		if input.key_down(Key::A) || input.key_down(Key::Left) {
			vel.x -= ROLL_SPEED;
		}
		if input.key_down(Key::D) || input.key_down(Key::Right) {
			vel.x += ROLL_SPEED;
		}
	}

	pub fn jump_if_pressed(&mut self, vel: &mut Vector, input: &Input) {
		if self.can_jump(input) {
			vel.y -= JUMP_HEIGHT;
			self.jump_key_released = false;
		}
	}

	fn can_jump(&self, input: &Input) -> bool {
		self.jump_key_released && (input.key_down(Key::W) || input.key_down(Key::Up))
	}

	pub fn set_jump_key_released(&mut self, input: &Input) {
		// TODO: Base this off of framestamp
		if !input.key_down(Key::W) {
			self.jump_key_released = true;
		}
	}
}
