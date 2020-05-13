use parser;
use password::Password;
use std::error;
use std::process;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    if let Err(e) = run() {
        println!("Application error: {}", e);
        process::exit(1);
    };

    println!("Completed program in: {} ms", now.elapsed().as_millis());
}

pub fn run() -> Result<(), Box<dyn error::Error>> {
    /*
        // Instruct the parser to read our file
        let filename: String = String::from("input.txt");

        // Turn raw strings to vector of strings
        let contents = parser::read_file(&filename)?;
        let claims = contents.lines().collect();
    */
    let mut count: i16 = 0;

    for i in 357253..=892942 {
        let password = Password::from(i as i32);
        match does_password_meet_criteria(&password) {
            true => count += 1,
            false => (),
        }
    }

    println!("Count of passwords that meet the criteria: {}", count);
    Ok(())
}

fn does_password_meet_criteria(password: &Password) -> bool {
    password.is_increasing_each_digit()
        && password.has_identical_adjacent_digits()
        && password.has_identical_adjacent_digits_in_pairs()
}
