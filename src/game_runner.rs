use crate::rolly_game::RollyGame;
use quicksilver::{Graphics, Input, Result, Timer, Window};

pub trait Step {
	fn step(&mut self, input: &Input, gfx: &mut Graphics, window: &Window) -> Result<()>;
}

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
}

impl Step for TimeStepper {
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

struct FrameStepper<'a> {
	game: RollyGame,
	next_pressed: &'a bool,
}

impl<'a> FrameStepper<'a> {
	#[allow(unused)]
	pub fn new(next_pressed: &'a bool) -> Self {
		Self {
			game: RollyGame::new(),
			next_pressed,
		}
	}
}

impl<'a> Step for FrameStepper<'a> {
	fn step(&mut self, input: &Input, gfx: &mut Graphics, window: &Window) -> Result<()> {
		if *self.next_pressed {
			self.game.update(&input);
			self.game.draw(gfx);
			gfx.present(&window)?;
		}
		Ok(())
	}
}
