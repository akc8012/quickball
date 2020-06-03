#[derive(Serialize, Deserialize)]
pub struct Config {
	pub step_mode: bool,
	pub load_art: bool,
}

impl Default for Config {
	fn default() -> Self {
		Config {
			step_mode: false,
			load_art: true,
		}
	}
}

pub fn load() -> Config {
	if cfg!(target_arch = "wasm32") {
		Config::default()
	} else {
		confy::load("quickball").expect("could not load config uWu")
	}
}
