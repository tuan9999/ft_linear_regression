use config::Config;
use csv::Reader;
use data::DataSet;
use linear_regression::LinearRegression;
use tracing::Level;
use utils::get_file_contents;
use std::{path::PathBuf, fs::File};

use crate::data::DataRecord;

pub mod config;
pub mod utils;
pub mod data;
pub mod linear_regression;

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
    
	let rdr = Reader::from_path(config.path).map_err(|e| tracing::error!("Error reading file: {e}"))?;

	let data = get_vector_of_data_records(rdr)?;

	let data = DataSet::new(data);

	let mut linear_regression = LinearRegression::new(data, None, config.gradient_descent_iterations);

	linear_regression.train();

	linear_regression.save_thetas_to_file(&config.path_to_save_thetas_file)?;
    Ok(())
}

fn get_vector_of_data_records(mut rdr: Reader<File>) -> Result<Vec<DataRecord>, ()> {
	let mut data = Vec::new();
	for result in rdr.deserialize() {
		let record: DataRecord = result.map_err(|e| tracing::error!("Error deserializing record: {e}"))?;
		data.push(record);
	}
	Ok(data)
}
