pub mod time_stepper;

use crate::{
	components::draw::{circle::*, image::*},
	config::Config,
	physics::colliders::{circle_bounds::CircleBounds, Colliders},
	player::Player,
};

use quicksilver::{
	geom::{Rectangle, Vector},
	graphics::{Color, Image},
	input::Key,
	Graphics, Input, Result,
};

pub struct Game {
	player: Player,
	colliders: Colliders,

	// TODO: Something more formalized to load resources: Provide a list of filenames, access a map via filename
	background: Option<Image>,
}

impl Game {
	pub async fn new(config: &Config, gfx: &Graphics, size: Vector) -> Result<Self> {
		let (background, ball) = if config.load_art {
			Self::load_images(gfx).await?
		} else {
			(None, None)
		};

		Ok(Game {
			background,
			player: Self::create_player(ball),
			colliders: Colliders::new(size, config.draw_colliders),
		})
	}

	// TODO: Return a slice or Vec of Option<Image>
	async fn load_images(gfx: &Graphics) -> Result<(Option<Image>, Option<Image>)> {
		let background = Image::load(gfx, "background.png").await?;
		let ball = Image::load(gfx, "ball.png").await?;

		Ok((Some(background), Some(ball)))
	}

	fn create_player(ball: Option<Image>) -> Player {
		Player::new(
			Box::new(CircleBounds::new((300, 20).into(), 16.)),
			match ball {
				Some(ball) => Box::new(DrawImageComponent::new(ball)),
				None => Box::new(DrawCircleComponent::new(Color::from_hex("4f30d9"))),
			},
		)
	}

	pub fn update(&mut self, input: &Input) {
		// print mouse location
		if input.key_down(Key::LControl) {
			println!("{}", input.mouse().location());
		}

		self.colliders.update(input);
		self.player.update(input, &self.colliders);

		if input.key_down(Key::Space) {
			self.player.reset();
		}
	}

	pub fn draw(&mut self, gfx: &mut Graphics, debug_draw: bool) {
		// background
		match &self.background {
			Some(background) => gfx.draw_image(background, Rectangle::new(Vector::ZERO, background.size())),
			None => gfx.clear(Color::from_hex("ade7ff")),
		}

		self.colliders.draw(gfx);
		self.player.draw(gfx, debug_draw);
	}
}
