mod player;
mod rolly_game;
use rolly_game::RollyGame;

use quicksilver::{
	input::{Event, Key},
	run, Graphics, Input, Result, Settings, Timer, Window,
};

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
	let mut game = RollyGame::new();
	let mut update_timer = Timer::time_per_second(60.0);
	let mut draw_timer = Timer::time_per_second(60.0);

	let mut running = true;
	while running {
		while let Some(event) = input.next_event().await {
			match event {
				Event::KeyboardInput(key) if key.key() == Key::Escape => running = false,
				_ => (),
			}
		}

		while update_timer.tick() {
			game.update(&input);
		}

		if draw_timer.exhaust().is_some() {
			game.draw(&mut gfx);
			gfx.present(&window)?;
		}
	}

	Ok(())
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
