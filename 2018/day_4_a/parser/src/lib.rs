use std::error;
use std::fs;
use std::path::PathBuf;
use std::env;


/// Given a filename, looks in the current working directory, opens the
/// specified filename and returns the contents if successful
pub fn read_file(filename: &str) -> Result<String, Box<dyn error::Error>> {
    let mut input_path: PathBuf = env::current_dir().unwrap();
    input_path.push(filename);

    let contents: String = fs::read_to_string(input_path)?;
    Ok(contents)
}

/// Human readable way to know which even is happening
pub enum Event {
    FallAsleep,
    WakeUp,
    ShiftStart
}

pub struct Record {

}