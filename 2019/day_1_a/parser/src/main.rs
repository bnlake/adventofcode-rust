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

/// Returning PathBuf because a Path is simply referencing a string/pathbuf
/// @url https://www.reddit.com/r/rust/comments/7mu7q1/is_working_with_paths_always_this_painful/
///
/// # Examples
///
/// ```rust
/// use std::env;
/// use parser::build_path_from_curr_env;
///
/// let filename: String = String::from("input.txt");
/// let mut control = std::env::current_dir().unwrap();
/// control.push(&filename);
///
/// assert_eq!(build_path_from_curr_env(&filename).unwrap(), control);
/// ```
pub fn build_path_from_curr_env(filename: &str) -> Result<PathBuf, Box<dyn error::Error>> {
    let mut path = env::current_dir()?;
    path.push(filename);

    Ok(path)
}