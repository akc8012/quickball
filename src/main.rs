mod rolly_game;
use rolly_game::RollyGame;

use quicksilver::{
	lifecycle::{run, Settings, State, Window},
	Result,
};

struct Game {
	game: RollyGame,
}

impl State for Game {
	fn new() -> Result<Game> {
		let game = RollyGame::new();
		Ok(Game { game })
	}

	fn update(&mut self, window: &mut Window) -> Result<()> {
		self.game.update(window, 1.0 / 60.0)
	}

	fn draw(&mut self, window: &mut Window) -> Result<()> {
		self.game.draw(window)
	}
}

fn main() {
	run::<Game>("RollyBall", (854, 480).into(), Settings::default());
}
