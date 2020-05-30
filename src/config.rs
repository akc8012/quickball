use confy;

#[derive(Serialize, Deserialize)]
pub struct Config {
	pub step_mode: bool,
}

impl Default for Config {
	fn default() -> Self {
		Config { step_mode: false }
	}
}

pub fn load() -> Config {
	confy::load("quickball").expect("could not load config uWu")
}
