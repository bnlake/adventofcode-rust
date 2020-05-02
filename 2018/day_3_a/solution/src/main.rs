use parser;
use parser::parse_line_into_rectangle;
use rectangle::Rectangle;
use std::collections::HashMap;
use std::error;
use std::process;
use utils::get_intersections_from_sweep_line;
use utils::{Event, EventType};

fn main() {
    if let Err(e) = run() {
        println!("Application error: {}", e);
        process::exit(1);
    };
}

pub fn run() -> Result<(), Box<dyn error::Error>> {
    // Instruct the parser to read our file
    let filename: String = String::from("input.txt");

    // Turn raw strings to vector of strings
    let contents = parser::read_file(&filename)?;
    let mut rectangles: HashMap<i32, Rectangle> = HashMap::new();

    for line in contents.lines() {
        let parsed_rect = parse_line_into_rectangle(&line);
        rectangles.insert(parsed_rect.id, parsed_rect);
    }

    match find_total_area_of_intersecting_rectangles(rectangles) {
        Ok(area) => println!("Total overlapping area is {} squared", area),
        Err(e) => println!("Couldn't calculate area: {}", e),
    }

    Ok(())
}

/// Given `[Rectangle]`, returns the total area: `i32` that two or more Rectangles intersect
/// with each other
pub fn find_total_area_of_intersecting_rectangles(
    rectangles: HashMap<i32, Rectangle>,
) -> Result<i32, Box<dyn error::Error>> {
    let mut sum_areas: i32 = 0;

    let mut events: Vec<Event> = Vec::new();
    // Generate all events from every rectangle. We can find the rectangle from the id
    for (&id, rectangle) in rectangles.iter() {
        // Push the left and right events to the sweet line
        events.push(Event {
            x: rectangle.left(),
            event_type: EventType::Start,
            rectangle_id: id,
        });
        events.push(Event {
            x: rectangle.right(),
            event_type: EventType::End,
            rectangle_id: id,
        });
    }

    // Sort the sweep line events so we can iterate
    events.sort_by_key(|x| x.x);

    let mut result: i32 = 0;
    let result_rectangles = get_intersections_from_sweep_line(rectangles, events);
    for rect in result_rectangles.unwrap() {
        result += rect.area();
    }

    Ok(result)
}
