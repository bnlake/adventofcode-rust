use std::error;
use std::fs;
use std::path::PathBuf;
use std::env;
use rectangle::Rectangle;
use point::Point;


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


/// Given a set pattern, parses a raw line of input into a Rectangle struct
///
/// # Examples
///
/// ```rust
/// use rectangle::Rectangle;
/// use point::Point;
/// use parser::parse_line_into_rectangle;
///
/// let test: String = String::from("#1 @ 1,3: 4x4");
/// let control: Rectangle = Rectangle {
///     id: 1,
///     p1: Point::new(1, 3),
///     p2: Point::new(5, 7),
/// };
///
/// let result: Rectangle = parse_line_into_rectangle(&test);
/// assert!(result == control);
/// ```
pub fn parse_line_into_rectangle(input: &str) -> Rectangle {
    // We'll rely on an external crate regex to simplify this solution
    use regex::Regex;
    let re = Regex::new(r"^#(?P<id>[0-9]+)\s@\s(?P<left>[0-9]+),(?P<top>[0-9]+):\s(?P<width>[0-9]+)x(?P<height>[0-9]+)$").unwrap();
    // Apply the regex and capture the match
    // @url https://docs.rs/regex/1.3.7/regex/struct.Regex.html#method.captures
    let captures = re.captures(input).unwrap();
    let left: i32 = captures["left"].parse().unwrap();
    let top: i32 = captures["top"].parse().unwrap();
    let width: i32 = captures["width"].parse().unwrap();
    let height: i32 = captures["height"].parse().unwrap();

    // Return the new rectangle
    let result: Rectangle = Rectangle {
        id: captures["id"].parse().unwrap(),
        p1: Point::new(left, top),
        p2: Point::new(left + width, top + height),
    };
    result
}