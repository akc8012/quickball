use quicksilver::{
	geom::{Rectangle, Vector},
	graphics::Color,
	input::Key,
	Graphics, Input,
};

pub struct RollyGame {
	player: Player,
}

struct Player {
	pos: Vector,
}

impl RollyGame {
	pub fn new() -> Self {
		let player = Player {
			pos: (300, 300).into(),
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
		// Paint a blue square at the given position
		gfx.fill_rect(
			&Rectangle::new(self.player.pos, Vector::new(64.0, 64.0)),
			Color::BLUE,
		);
	}
}
