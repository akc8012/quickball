use crate::{collider::Collider, player::Player};
use quicksilver::{
	geom::{Rectangle, Vector},
	graphics::Image,
	input::Key,
	Graphics, Input, Result,
};

pub struct RollyGame {
	player: Player,
	colliders: Vec<Collider>,
	background: Image,
}

impl RollyGame {
	// TODO: window size as RollyGame field
	pub async fn new(gfx: &Graphics, size: Vector) -> Result<Self> {
		let ground = Collider::new((0.0, size.y - 20.0), (size.x, 32.0));

		Ok(RollyGame {
			player: Player::new(),
			colliders: vec![ground],
			background: Image::load(gfx, "background.png").await?,
		})
	}

	pub fn update(&mut self, input: &Input, size: Vector) {
		self.player.update(input, &self.colliders, size);

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
