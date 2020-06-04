use crate::{
	collider::{Collide, PointCollider, RectangleCollider},
	config::Config,
	player::Player,
};
use quicksilver::{
	geom::{Rectangle, Vector},
	graphics::{Color, Image},
	input::Key,
	Graphics, Input, Result,
};

pub struct RollyGame {
	player: Player,
	colliders: Vec<Box<dyn Collide>>,
	background: Option<Image>,
	ball: Option<Image>, // TODO: Something more formalized to load resources: A method loading a map of images?
}

impl RollyGame {
	// TODO: window size as RollyGame field
	pub async fn new(config: &Config, gfx: &Graphics, size: Vector) -> Result<Self> {
		Ok(RollyGame {
			player: Player::new(),
			colliders: RollyGame::create_colliders(size),
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

	fn create_colliders(size: Vector) -> Vec<Box<dyn Collide>> {
		let mut colliders: Vec<Box<dyn Collide>> = Vec::new();

		// ground
		colliders.push(Box::new(RectangleCollider::new(
			(0.0, size.y - 20.0),
			(size.x, 32.0),
		)));

		// platform
		colliders.push(Box::new(RectangleCollider::new((525, 400), (128, 10))));

		// points
		for x in 0..300 {
			for y in 380..383 {
				colliders.push(Box::new(PointCollider::new((x, y).into())));
			}
		}

		colliders
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

		self.player.draw(&self.ball, gfx);
	}
}
