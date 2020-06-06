use crate::player::Player;
use quicksilver::{input::Key, Input};

#[derive(Copy, Clone)]
pub struct PlayerInput {
	pub jump_key_released: bool,
}

impl PlayerInput {
	pub fn new() -> Self {
		PlayerInput {
			jump_key_released: true,
		}
	}

	pub fn update(self, player: &mut Player, input: &Input) {
		self.roll(player, input);
	}

	fn roll(self, player: &mut Player, input: &Input) {
		const ROLL_SPEED: f32 = 4.;
		player.vel.x = 0.;

		if input.key_down(Key::A) || input.key_down(Key::Left) {
			player.vel.x -= ROLL_SPEED;
		}
		if input.key_down(Key::D) || input.key_down(Key::Right) {
			player.vel.x += ROLL_SPEED;
		}
	}

	pub fn can_jump(&self, input: &Input) -> bool {
		self.jump_key_released && (input.key_down(Key::W) || input.key_down(Key::Up))
	}

	pub fn set_jump_key_released(&mut self, input: &Input) {
		// TODO: Base this off of framestamp
		if !input.key_down(Key::W) {
			self.jump_key_released = true;
		}
	}
}