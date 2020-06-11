use super::Game;
use crate::config::Config;
use quicksilver::{Graphics, Input, Result, Timer, Window};

pub struct TimeStepper {
	game: Game,
	update_timer: Timer,
	draw_timer: Timer,
}

impl TimeStepper {
	pub fn new(game: Game) -> Self {
		Self {
			game,
			update_timer: Timer::time_per_second(60.0),
			draw_timer: Timer::time_per_second(60.0),
		}
	}

	pub fn timed_step(
		&mut self,
		input: &Input,
		gfx: &mut Graphics,
		window: &Window,
		config: &Config,
	) -> Result<()> {
		while self.update_timer.tick() {
			self.game.update(&input);
		}

		if self.draw_timer.exhaust().is_some() {
			self.game.draw(gfx, config.debug_draw);
			gfx.present(&window)?;
		}

		Ok(())
	}

	pub fn step(
		&mut self,
		input: &Input,
		gfx: &mut Graphics,
		window: &Window,
		config: &Config,
	) -> Result<()> {
		self.game.update(&input);
		self.game.draw(gfx, config.debug_draw);
		gfx.present(&window)
	}
}
