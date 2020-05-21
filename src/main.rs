mod rolly_game;
use rolly_game::RollyGame;

use quicksilver::{run, Graphics, Input, Result, Settings, Timer, Window};

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
	let mut game = RollyGame::new();
	let mut update_timer = Timer::time_per_second(60.0);
	let mut draw_timer = Timer::time_per_second(60.0);

	loop {
		while let Some(_) = input.next_event().await {}

		while update_timer.tick() {
			game.update(&input);
		}

		if draw_timer.exhaust().is_some() {
			game.draw(&mut gfx);
			gfx.present(&window)?;
		}
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
