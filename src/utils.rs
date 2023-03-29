use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

pub fn get_file_contents(path: &Path) -> Result<String, ()> {
    let mut file = File::open(path).map_err(|e| tracing::error!("Error opening file: {e}"))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| tracing::error!("Error reading file: {e}"))?;
    tracing::info!("Read file: {path:?}");
    Ok(contents)
}

fn create_file(path: &Path) -> Result<File, ()> {
    let file = File::create(path).map_err(|e| tracing::error!("Error creating file: {e}"))?;
    tracing::info!("Created file: {path:?}");
    Ok(file)
}

pub fn save_contents_to_file(path: &Path, contents: &str) -> Result<(), ()> {
    let mut file = create_file(path)?;
    file.write_all(contents.as_bytes())
        .map_err(|e| tracing::error!("Error writing to file: {e}"))?;
    tracing::info!("Wrote to file: {path:?}");
    Ok(())
}
