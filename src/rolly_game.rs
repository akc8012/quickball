use crate::{collider::RectangleCollider, config::Config, player::Player};
use quicksilver::{
	geom::{Rectangle, Vector},
	graphics::{Color, Image},
	input::Key,
	Graphics, Input, Result,
};

pub struct RollyGame {
	player: Player,
	colliders: Vec<RectangleCollider>,
	background: Option<Image>,
	ball: Option<Image>, // TODO: Something more formalized to load resources: A method loading a map of images?
}

impl RollyGame {
	// TODO: window size as RollyGame field
	pub async fn new(config: &Config, gfx: &Graphics, size: Vector) -> Result<Self> {
		let ground = RectangleCollider::new((0.0, size.y - 20.0), (size.x, 32.0));
		let platform = RectangleCollider::new((525, 400), (128, 10));

		Ok(RollyGame {
			player: Player::new(),
			colliders: vec![ground, platform],
			// TODO: clean up
			background: if config.load_art {
				Some(Image::load(gfx, "background.png").await?)
			} else {
				None
			},
			ball: if config.load_art {
				Some(Image::load(gfx, "ball.png").await?)
			} else {
				None
			},
		})
	}

	pub fn update(&mut self, input: &Input, size: Vector) {
		self.player.update(input, &self.colliders, size);

		if input.key_down(Key::Space) {
			self.player.reset();
		}
	}

	pub fn draw(&mut self, gfx: &mut Graphics) {
		// background
		if let Some(background) = &self.background {
			gfx.draw_image(background, Rectangle::new(Vector::ZERO, background.size()));
		} else {
			gfx.clear(Color::from_hex("ade7ff"));
		}

		for collider in &self.colliders {
			collider.draw(gfx);
		}

		// points
		for x in 0..10 {
			gfx.draw_point((x, 10).into(), Color::GREEN);
		}

		self.player.draw(&self.ball, gfx);
	}
}
