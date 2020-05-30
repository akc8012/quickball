use crate::player::Player;
use quicksilver::{graphics::Color, input::Key, Graphics, Input};

pub struct RollyGame {
	player: Player,
}

impl RollyGame {
	pub fn new() -> Self {
		RollyGame {
			player: Player::new(),
		}
	}

	pub fn update(&mut self, input: &Input) {
		self.player.update(input);

		if input.key_down(Key::Space) {
			self.player.reset();
		}
	}

	pub fn draw(&mut self, gfx: &mut Graphics) {
		gfx.clear(Color::from_hex("ade7ff"));
		self.player.draw(gfx);
	}
}
