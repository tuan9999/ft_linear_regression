use std::path::PathBuf;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
	pub path_to_save_thetas_file: PathBuf,
	pub path_to_test_file: PathBuf,
	pub graph_path: PathBuf,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ThetaFile {
	pub theta_zero: f64,
	pub theta_one: f64,
}
