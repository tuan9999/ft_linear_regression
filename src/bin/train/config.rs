use std::path::PathBuf;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
	pub path: PathBuf,
	pub path_to_save_thetas_file: PathBuf,
	pub gradient_descent_iterations: usize,
}
