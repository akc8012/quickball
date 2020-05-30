mod collider;
mod game_runner;
mod player;
mod raycast;
mod rolly_game;
use game_runner::*;

use quicksilver::{
	input::{Event, Key},
	run, Graphics, Input, Result, Settings, Window,
};

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
	let mut game_runner = TimeStepper::new();

	let mut running = true;
	let mut _next_pressed = false;

	while running {
		_next_pressed = false;

		while let Some(event) = input.next_event().await {
			if let Event::KeyboardInput(key) = event {
				if key.key() == Key::Escape {
					running = false
				}
				if key.key() == Key::N && key.is_down() {
					_next_pressed = true
				}
			}
		}

		game_runner.step(&input, &mut gfx, &window)?;
	}

	Ok(())
}

fn main() {
	run(
		Settings {
			title: "Quickball",
			size: (854, 480).into(),
			..Settings::default()
		},
		app,
	);
}
