// Example 8: Input
// Respond to user keyboard and mouse input onscreen
use quicksilver::{
	geom::{Circle, Rectangle, Vector},
	graphics::Color,
	input::Key,
	run, Graphics, Input, Result, Settings, Window,
};

struct Player {
	pos: Vector
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
	// Keep track of the position of the square
	let mut player = Player { pos: (300, 300).into() };

	loop {
		while let Some(_) = input.next_event().await {}
		// Check the state of the keys, and move the square accordingly
		const SPEED: f32 = 2.0;
		if input.key_down(Key::A) {
			player.pos.x -= SPEED;
		}
		if input.key_down(Key::D) {
			player.pos.x += SPEED;
		}
		if input.key_down(Key::W) {
			player.pos.y -= SPEED;
		}
		if input.key_down(Key::S) {
			player.pos.y += SPEED;
		}

		gfx.clear(Color::from_hex("ade7ff"));
		// Paint a blue square at the given position
		gfx.fill_rect(
			&Rectangle::new(player.pos, Vector::new(64.0, 64.0)),
			Color::BLUE,
		);
		// Paint a red square at the mouse position
		gfx.fill_circle(&Circle::new(input.mouse().location(), 32.0), Color::RED);
		gfx.present(&window)?;
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
