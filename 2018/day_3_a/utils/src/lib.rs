use point::Point;
use rectangle::Rectangle;
use std::borrow::Borrow;
use std::collections::HashMap;

pub enum EventType {
    Start,
    End,
}

pub struct Event {
    pub x: i32,
    pub event_type: EventType,
    pub rectangle_id: i32,
}

pub fn get_intersections_from_sweep_line(
    rectangles: HashMap<i32, Rectangle>,
    events: Vec<Event>,
) -> Option<Vec<Rectangle>> {
    let mut results: Vec<Rectangle> = Vec::new();

    // We'll use a hashmap like a stack of rectangles that the sweep line
    // is currently checking
    let mut range_stack: HashMap<i32, &Rectangle> = HashMap::new();

    // Now for every event, let's either add or remove a rectangle from the hashmap
    for (i, event) in events.iter().enumerate() {
        match event.event_type {
            EventType::Start => {
                range_stack.insert(
                    event.rectangle_id.clone(),
                    rectangles.get(event.rectangle_id.borrow()).clone().unwrap(),
                );
            }
            EventType::End => {
                range_stack.remove(event.rectangle_id.borrow());
            }
        }

        // No matter the event, it's still the beginning of a section so let's check
        // and retrieve intersected rectangles
        if range_stack.len() >= 1 {
            match extract_intersecting_rectangles_in_range(&range_stack, event.x, events[i + 1].x) {
                Some(T) => {
                    for rect in T {
                        results.push(rect);
                    }
                }
                None => (),
            }
        }
    }

    Some(results)
}


/// An overcomplicated way to build rectangles from intersecting areas because I'm bored with this
/// days AoC!
///
/// # Examples
/// rust
/// use rectangle::Rectangle;
/// use point::Point;
/// use std::collections::HashMap;
/// use utils::extract_intersecting_rectangles_in_range;
///
/// let left: i32 = 1;
/// let right: i32 = 4;
/// let x1: Rectangle = Rectangle::new(1, Point::new(left, 0), Point::new(right, 4));
/// let x2: Rectangle = Rectangle::new(2, Point::new(left, 2), Point::new(right, 8));
/// let y1: Rectangle = Rectangle::new(3, Point::new(left, 6), Point::new(right, 8));
///
/// let mut stack: HashMap<i32, &Rectangle> = HashMap::new();
/// stack.insert(1, &x1);
/// stack.insert(2, &x2);
/// stack.insert(3, &y1);
///
/// assert_eq!(extract_intersecting_rectangles_in_range(&stack, left, right), Some(1));
/// ```
pub fn extract_intersecting_rectangles_in_range(
    range_stack: &HashMap<i32, &Rectangle>,
    left: i32,
    right: i32,
) -> Option<Vec<Rectangle>> {
    if range_stack.len() < 1 {
        None
    } else {
        let mut results: Vec<Rectangle> = Vec::new();

        // We need to turn our rectangles into events
        // our enum states x but we'll treat it as y
        let mut sweep_line: Vec<Event> = Vec::new();
        for (&id, &rectangle) in range_stack {
            sweep_line.push(Event {
                x: rectangle.top(),
                event_type: EventType::Start,
                rectangle_id: rectangle.id,
            });
            sweep_line.push(Event {
                x: rectangle.bottom(),
                event_type: EventType::End,
                rectangle_id: rectangle.id,
            })
        }

        sweep_line.sort_by_key(|x| x.x);

        let mut sweep_stack: HashMap<i32, i32> = HashMap::new();
        let mut x_start: i32 = 0;
        for (i, event) in sweep_line.iter().enumerate() {
            match event.event_type {
                EventType::Start => {
                    sweep_stack.insert(event.rectangle_id, event.x);

                    if sweep_stack.len() > 1 {
                        x_start = event.x;
                    }
                }
                EventType::End => {
                    sweep_stack.remove(event.rectangle_id.borrow());

                    if sweep_stack.len() < 2 {
                        results.push(Rectangle::new(
                            0,
                            Point::new(left, x_start),
                            Point::new(right, event.x),
                        ));

                        x_start = 0;
                    }
                }
            }
        }

        if results.len() > 0 {
            Some(results)
        } else {
            None
        }
    }
}
