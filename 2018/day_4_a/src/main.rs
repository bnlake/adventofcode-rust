use std::error;
use std::process;

use parser;
use regex;
use timestamp::Timestamp;

fn main() {
    if let Err(e) = run() {
        println!("Application error: {}", e);
        process::exit(1);
    };
}

fn run() -> Result<(), Box<dyn error::Error>> {
    // Instruct the parser to read our file
    let filename: String = String::from("input.txt");

    // Turn raw strings to vector of strings
    let contents = parser::read_file(&filename)?;
    let raw_records = contents.lines().collect();

    println!("Answer: {}", find_solution(raw_records));

    Ok(())
}

fn find_solution(raw_records: Vec<&str>) -> i32 {
    let result = 0;

    let re = regex::Regex::new(r"\[(.+)\]\s(.+)").unwrap();

    // input may not be sorted
    let mut sorted_raw_records: Vec<(Timestamp, &str)> = Vec::new();
    // Split the timestamp from the event
    for record in raw_records {
        let captures = re.captures(&record).unwrap();
        let timestamp_raw = captures.get(1).map(|x| x.as_str()).unwrap();
        let event_raw = captures.get(2).map(|x| x.as_str()).unwrap();

        sorted_raw_records.push((Timestamp::from(timestamp_raw), event_raw));
    }
    sorted_raw_records.sort_by(|x,y| Timestamp::from(&x(0)).partial_cmp(&Timestamp::from(&y(0))).unwrap());
    return result;
}
