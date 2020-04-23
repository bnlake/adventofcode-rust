use std::error;
use std::fs;
use std::path::PathBuf;

pub fn read_file(path: PathBuf) -> Result<String, Box<dyn error::Error>> {
    let contents: String = fs::read_to_string(path)?;

    Ok(contents)
}