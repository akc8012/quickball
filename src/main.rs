mod collider;
mod config;
mod player;
mod raycast;
mod rolly_game;
mod time_stepper;

#[macro_use]
extern crate serde_derive;

use quicksilver::{
	input::{Event, Key},
	run, Graphics, Input, Result, Settings, Window,
};

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
	let mut time_stepper = time_stepper::TimeStepper::new(&gfx).await?;
	let mut running = true;
	let mut config = config::load();

	while running {
		while let Some(event) = input.next_event().await {
			if let Event::KeyboardInput(key) = event {
				if key.key() == Key::Escape {
					running = false
				}
				if key.key() == Key::L && key.is_down() {
					config = config::load();
				}
				if config.step_mode && key.key() == Key::N && key.is_down() {
					time_stepper.step(&input, &mut gfx, &window)?;
				}
			}
		}

		if !config.step_mode {
			time_stepper.timed_step(&input, &mut gfx, &window)?;
		}
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
