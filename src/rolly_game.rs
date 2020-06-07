use crate::{config::Config, physics::*, player::Player};

use quicksilver::{
	geom::{Rectangle, Vector},
	graphics::{Color, Image},
	input::Key,
	Graphics, Input, Result,
};

pub struct RollyGame {
	player: Player,
	colliders: Vec<Box<dyn Bounds>>,
	background: Option<Image>,
	ball: Option<Image>, // TODO: Something more formalized to load resources: A method loading a map of images?
}

impl RollyGame {
	// TODO: window size as RollyGame field
	pub async fn new(config: &Config, gfx: &Graphics, size: Vector) -> Result<Self> {
		let (background, ball) = if config.load_art {
			let background = Image::load(gfx, "background.png").await?;
			let ball = Image::load(gfx, "ball.png").await?;

			(Some(background), Some(ball))
		} else {
			(None, None)
		};

		Ok(RollyGame {
			player: Player::new(Box::new(CircleBounds::new((300, 20).into(), 16.))),
			colliders: RollyGame::create_colliders(size),
			background,
			ball,
		})
	}

	fn create_colliders(size: Vector) -> Vec<Box<dyn Bounds>> {
		let mut colliders: Vec<Box<dyn Bounds>> = Vec::new();

		// ground
		colliders.push(Box::new(RectangleBounds::new(
			(0.0, size.y - 20.0),
			(size.x, 32.0),
		)));

		// platform
		colliders.push(Box::new(RectangleBounds::new((525, 400), (128, 10))));

		// points
		for x in 0..300 {
			for y in 380..383 {
				colliders.push(Box::new(PointBounds::new((x, y).into())));
			}
		}

		colliders
	}

	pub fn update(&mut self, input: &Input) {
		// print mouse location
		if input.key_down(Key::LControl) {
			println!("{}", input.mouse().location());
		}

		if input.mouse().left() {
			self.colliders
				.push(Box::new(PointBounds::new(input.mouse().location())));

			for x in -5..5 {
				for y in -5..5 {
					self.colliders.push(Box::new(PointBounds::new(
						input.mouse().location() + (x, y).into(),
					)));
				}
			}
		}

		self.player.update(input, &self.colliders);

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
