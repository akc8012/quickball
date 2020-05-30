mod collider;
mod player;
mod raycast;
mod rolly_game;
use rolly_game::RollyGame;

use quicksilver::{
	input::{Event, Key},
	run, Graphics, Input, Result, Settings, Timer, Window,
};

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
	let mut gameloop = GameTimerStepper::new();

	let mut running = true;
	let mut _next_pressed;

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

		gameloop.step(&input, &mut gfx, &window)?;
	}

	Ok(())
}

trait Step {
	fn step(&mut self, input: &Input, gfx: &mut Graphics, window: &Window) -> Result<()>;
}

struct GameTimerStepper {
	game: RollyGame,
	update_timer: Timer,
	draw_timer: Timer,
}

impl GameTimerStepper {
	fn new() -> Self {
		Self {
			game: RollyGame::new(),
			update_timer: Timer::time_per_second(60.0),
			draw_timer: Timer::time_per_second(60.0),
		}
	}
}

impl Step for GameTimerStepper {
	fn step(&mut self, input: &Input, gfx: &mut Graphics, window: &Window) -> Result<()> {
		while self.update_timer.tick() {
			self.game.update(&input);
		}

		if self.draw_timer.exhaust().is_some() {
			self.game.draw(gfx);
			gfx.present(&window)?;
		}

		Ok(())
	}
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
