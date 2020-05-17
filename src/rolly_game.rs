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
	radius: f32,
}

impl RollyGame {
	pub fn new() -> Self {
		let player = Player {
			pos: (300, 300).into(),
			radius: 16.0,
		};
		RollyGame { player }
	}

	pub fn update(&mut self, input: &Input) {
		const SPEED: f32 = 2.0;
		if input.key_down(Key::A) {
			self.player.pos.x -= SPEED;
		}
		if input.key_down(Key::D) {
			self.player.pos.x += SPEED;
		}
		if input.key_down(Key::W) {
			self.player.pos.y -= SPEED;
		}
		if input.key_down(Key::S) {
			self.player.pos.y += SPEED;
		}
	}

	pub fn draw(&mut self, gfx: &mut Graphics) {
		gfx.clear(Color::from_hex("ade7ff"));

		gfx.fill_circle(
			&Circle::new(self.player.pos, self.player.radius),
			Color::from_hex("4f30d9"),
		);
	}
}
