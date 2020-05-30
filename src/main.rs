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
	let mut gameloop = GameRunner::new();

	let mut running = true;
	let mut next_pressed;

	while running {
		next_pressed = false;

		while let Some(event) = input.next_event().await {
			if let Event::KeyboardInput(key) = event {
				if key.key() == Key::Escape {
					running = false
				}
				if key.key() == Key::N && key.is_down() {
					next_pressed = true
				}
			}
		}

		gameloop.run(next_pressed, &input, &mut gfx, &window)?;
	}

	Ok(())
}

struct GameRunner {
	game: RollyGame,
	step_mode: bool,
	update_timer: Timer,
	draw_timer: Timer,
}

impl GameRunner {
	// TODO: two constructors to handle loop or step (or pass in a flag to one)
	fn new() -> Self {
		Self {
			game: RollyGame::new(),
			step_mode: false,
			update_timer: Timer::time_per_second(60.0),
			draw_timer: Timer::time_per_second(60.0),
		}
	}

	pub fn run(
		&mut self,
		next_pressed: bool,
		input: &Input,
		gfx: &mut Graphics,
		window: &Window,
	) -> Result<()> {
		if self.step_mode {
			if next_pressed {
				self.step(&input, gfx, &window)?;
			}
		} else {
			self.timestep(&input, gfx, &window)?;
		}

		Ok(())
	}

	fn timestep(&mut self, input: &Input, gfx: &mut Graphics, window: &Window) -> Result<()> {
		while self.update_timer.tick() {
			self.game.update(&input);
		}

		if self.draw_timer.exhaust().is_some() {
			self.game.draw(gfx);
			gfx.present(&window)?;
		}

		Ok(())
	}

	fn step(&mut self, input: &Input, gfx: &mut Graphics, window: &Window) -> Result<()> {
		self.game.update(&input);
		self.game.draw(gfx);
		gfx.present(&window)
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
