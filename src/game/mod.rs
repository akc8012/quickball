pub mod time_stepper;

use crate::{
	components::draw::{circle::*, image::*, DrawComponent},
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
			let background = Image::load(gfx, "background.png").await?;
			let ball = Image::load(gfx, "ball.png").await?;

			(Some(background), Some(ball))
		} else {
			(None, None)
		};

		let draw: Box<dyn DrawComponent> = match ball {
			Some(ball) => Box::new(DrawImageComponent::new(ball)),
			None => Box::new(DrawCircleComponent::new()),
		};

		Ok(Game {
			player: Player::new(Box::new(CircleBounds::new((300, 20).into(), 16.)), draw),
			colliders: Colliders::new(size),
			background,
		})
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

	pub fn draw(&mut self, gfx: &mut Graphics) {
		// background
		if let Some(background) = &self.background {
			gfx.draw_image(background, Rectangle::new(Vector::ZERO, background.size()));
		} else {
			gfx.clear(Color::from_hex("ade7ff"));
		}

		self.colliders.draw(gfx);
		self.player.draw(gfx);
	}
}
