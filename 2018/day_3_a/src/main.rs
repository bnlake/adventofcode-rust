mod parser;
mod calculator;

use std::error;
use std::process;
use std::env;
use std::path;

fn main() {
	if let Err(e) = run() {
		println!("Application error: {}", e);
		process::exit(1);
	};
}


pub fn run() -> Result<(), Box<dyn error::Error>> {
	// Instruct the parser to read our file
	let filename: String = String::from("input.txt");
	let mut input_path: path::PathBuf = env::current_dir().unwrap();
	input_path.push(&filename);

	let contents = parser::read_file(input_path)?;

	// todo implement algorithm

	Ok(())
}


// Returning PathBuf because a Path is simply referencing a string/pathbuf
// @url https://www.reddit.com/r/rust/comments/7mu7q1/is_working_with_paths_always_this_painful/
pub fn build_path_from_curr_env(filename: &str) -> Result<path::PathBuf, Box<dyn error::Error>> {
	let mut path = env::current_dir()?;
	path.push(filename);

	Ok(path)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn returns_path_to_curr_dir() {
		let filename: String = String::from("input.txt");
		let mut control = env::current_dir().unwrap();
		control.push(&filename);

		assert_eq!(build_path_from_curr_env(&filename).unwrap(), control);
	}
}