use quicksilver::{
	geom::{Circle, Vector},
	graphics::Color,
	input::Key,
	Graphics, Input,
};

pub struct RollyGame {
	player: Player,
}

struct Player {
	pos: Vector,
	vel: Vector,
	radius: f32,
}

impl RollyGame {
	pub fn new() -> Self {
		let player = Player {
			pos: (300, 20).into(),
			vel: Vector::ZERO,
			radius: 16.0,
		};
		RollyGame { player }
	}

	fn reset(&mut self) {
		self.player.pos = (300, 20).into();
		self.player.vel = Vector::ZERO;
	}

	pub fn update(&mut self, input: &Input) {
		if input.key_down(Key::Space) {
			self.reset();
		}

		const SPEED: f32 = 2.0;
		const GRAVITY: f32 = 6.0;

		// TODO: DELTA_TIME
		self.player.vel.x = 0.0;
		self.player.vel.y += GRAVITY;

		if input.key_down(Key::A) {
			self.player.vel.x -= SPEED;
		}
		if input.key_down(Key::D) {
			self.player.vel.x += SPEED;
		}

		self.player.pos += self.player.vel;
	}

	pub fn draw(&mut self, gfx: &mut Graphics) {
		gfx.clear(Color::from_hex("ade7ff"));

		gfx.fill_circle(
			&Circle::new(self.player.pos, self.player.radius),
			Color::from_hex("4f30d9"),
		);
	}
}
