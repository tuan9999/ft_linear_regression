use std::path::PathBuf;

use config::{Config, ThetaFile};
use csv::Reader;
use ft_linear_regression::{utils::get_file_contents, data::{get_vector_of_data_records, DataSet}};
use tracing::Level;

mod config;

#[derive(Debug, clap::Parser)]
pub struct Options {
    /// Path to the config file.
    #[clap(long, short)]
    #[clap(required = true)]
    config: PathBuf,
}

fn main() {
    if let Err(()) = do_main(clap::Parser::parse()) {
        std::process::exit(1)
    }
}

fn do_main(options: Options) -> Result<(), ()> {
	tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let file_contents = get_file_contents(&options.config)?;
    let config: Config = serde_yaml::from_str(&file_contents).map_err(|e| tracing::error!("Error parsing config file: {e}"))?;

	let file_contents = get_file_contents(&config.path_to_save_thetas_file)?;
    let thetas: ThetaFile = serde_yaml::from_str(&file_contents).map_err(|e| tracing::error!("Error parsing config file: {e}"))?;

	let rdr = Reader::from_path(config.path_to_test_file).map_err(|e| tracing::error!("Error reading file: {e}"))?;

	let data = get_vector_of_data_records(rdr)?;

	let data = DataSet::new(data);

	let mut linear_regression = ft_linear_regression::linear_regression::LinearRegression::new(data.clone(), None, 0);
	linear_regression.set_thetas(thetas.theta_zero, thetas.theta_one);

	for record in data {
		let prediction = linear_regression.predict(record.km);
		tracing::info!("Predicted price: {}, actual price: {}", prediction, record.price);
	}
	Ok(())
}
