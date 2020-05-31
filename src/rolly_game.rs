use crate::player::Player;
use quicksilver::{
	geom::{Rectangle, Vector},
	graphics::Image,
	input::Key,
	Graphics, Input, Result,
};

pub struct RollyGame {
	player: Player,
	background: Image,
}

impl RollyGame {
	pub async fn new(gfx: &Graphics) -> Result<Self> {
		Ok(RollyGame {
			player: Player::new(),
			background: Image::load(gfx, "background.png").await?,
		})
	}

	pub fn update(&mut self, input: &Input, size: Vector) {
		self.player.update(input, size);

		if input.key_down(Key::Space) {
			self.player.reset();
		}
	}

	pub fn draw(&mut self, gfx: &mut Graphics) {
		let region = Rectangle::new(Vector::ZERO, self.background.size());
		gfx.draw_image(&self.background, region);

		self.player.draw(gfx);
	}
}
