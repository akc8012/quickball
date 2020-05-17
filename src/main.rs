mod rolly_game;
use rolly_game::RollyGame;

use quicksilver::{run, Graphics, Input, Result, Settings, Window};

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
	let mut game = RollyGame::new();

	loop {
		while let Some(_) = input.next_event().await {}

		game.update(&input, 1.0 / 60.0);
		game.draw(&mut gfx);

		gfx.present(&window)?;
	}
}

fn main() {
	run(
		Settings {
			title: "Input Example",
			size: (854, 480).into(),
			..Settings::default()
		},
		app,
	);
}
