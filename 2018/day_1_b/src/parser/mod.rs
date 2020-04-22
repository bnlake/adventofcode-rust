use std::error;
use std::fs;
use std::path::PathBuf;

pub fn read_file(path: PathBuf) -> Result<String, Box<dyn error::Error>> {
    let contents: String = fs::read_to_string(path)?;

    Ok(contents)
}

// We need to parse the lines of strings to integers
pub fn parse_string_to_int(input: &str) -> i32 {
    let result: i32 = input.parse::<i32>().unwrap();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_to_signed_int() {
        let test: String = String::from("-10");
        let control: i32 = -10;

        assert_eq!(parse_string_to_int(&test), control);
    }
}
