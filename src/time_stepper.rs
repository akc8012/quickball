use crate::rolly_game::RollyGame;
use quicksilver::{Graphics, Input, Result, Timer, Window};

pub struct TimeStepper {
	game: RollyGame,
	update_timer: Timer,
	draw_timer: Timer,
}

impl TimeStepper {
	pub fn new() -> Self {
		Self {
			game: RollyGame::new(),
			update_timer: Timer::time_per_second(60.0),
			draw_timer: Timer::time_per_second(60.0),
		}
	}

	pub fn timed_step(&mut self, input: &Input, gfx: &mut Graphics, window: &Window) -> Result<()> {
		while self.update_timer.tick() {
			self.game.update(&input, window.size());
		}

		if self.draw_timer.exhaust().is_some() {
			self.game.draw(gfx);
			gfx.present(&window)?;
		}

		Ok(())
	}

	pub fn step(&mut self, input: &Input, gfx: &mut Graphics, window: &Window) -> Result<()> {
		self.game.update(&input, window.size());
		self.game.draw(gfx);
		gfx.present(&window)
	}
}
