use std::{fs::File, path::Path, io::Read};

pub fn get_file_contents(path: &Path) -> Result<String, ()> {
	let mut file = File::open(path).map_err(|e| tracing::error!("Error opening file: {e}"))?;
	let mut contents = String::new();
	file.read_to_string(&mut contents).map_err(|e| tracing::error!("Error reading file: {e}"))?;
	tracing::info!("Read file: {path:?}");
	Ok(contents)
}