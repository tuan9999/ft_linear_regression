use config::Config;
use csv::Reader;
use ft_linear_regression::{utils::get_file_contents, data::{DataSet, get_vector_of_data_records}, linear_regression::LinearRegression};
use tracing::Level;
use std::path::PathBuf;

pub mod config;

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

	let mut linear_regression = LinearRegression::new(data, None, config.gradient_descent_iterations, config.graph_path);

	linear_regression.train();

	linear_regression.save_thetas_to_file(&config.path_to_save_thetas_file)?;

	linear_regression.plot_data()
}
