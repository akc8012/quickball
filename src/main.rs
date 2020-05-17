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
			let delta_time = update_timer.remaining();
			if let Some(delta_time) = delta_time {
				println!("dt: {:?}", delta_time.as_secs_f32());
			}
		}

		if draw_timer.tick() {
			game.update(&input);
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
