use parser;
use std::collections::{HashMap, HashSet};
use std::error;
use std::process;

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
    let claims = contents.lines().collect();

    match find_total_square_inches_of_overlapping_claims(claims) {
        Ok(id) => println!("The only non intersecting claim id is: {}", id),
        Err(e) => println!("Couldn't calculate area: {}", e),
    }

    Ok(())
}

pub fn find_total_square_inches_of_overlapping_claims(
    raw_claims: Vec<&str>,
) -> Result<i32, Box<dyn error::Error>> {
    let mut claims = HashMap::new();
    let mut intersecting_claims = HashSet::new();
    let mut all_claims = HashSet::new();

    for raw_claim in raw_claims.iter() {
        let temp: Vec<i32> = raw_claim
            .split(|x| x == ' ' || x == '@' || x == ',' || x == ':' || x == 'x' || x == '#')
            .filter_map(|x| x.parse().ok())
            .collect();
        if let [id, left, top, width, height] = temp[..] {
            for x in left..left + width {
                for y in top..top + height {
                    all_claims.insert(id);

                    // Claims will be a first come first serve for every coordinate.
                    // It's how we'll check if a claim id has already taken ownership
                    // for a coordinate
                    if !claims.contains_key(&(x, y)) {
                        claims.insert((x, y), id);
                    } else {
                        // Since a claim has already taken over this coordinate, there is an
                        // intersection. We need to keep track of both intersection ids
                        intersecting_claims.insert(claims[&(x,y)]);
                        intersecting_claims.insert(id);
                    }
                }
            }
        }
    }

    // The puzzle wants the id for the only claim that isn't part of an intersection.
    let result = all_claims.difference(&intersecting_claims).next();
    Ok(result.unwrap().clone())
}

// fn example_from_reddit() {
//     let data = include_str!("input.txt");
//     let c = data.lines().collect::<Vec<_>>();
//     let mut claims = HashMap::new();
//     let mut claim_names = HashMap::new();
//     let mut intersecting = HashSet::new();
//     let mut all = HashSet::new();
//     for i in c.iter() {
//         let r = i
//             .split(|c| c == ' ' || c == '@' || c == ',' || c == ':' || c == 'x' || c == '#')
//             .filter_map(|c| c.parse::<usize>().ok())
//             .collect::<Vec<_>>();
//         for i in r[1]..r[1] + r[3] {
//             for j in r[2]..r[2] + r[4] {
//                 *claims.entry((i, j)).or_insert(0) += 1;
//                 all.insert(r[0]);
//                 if !claim_names.contains_key(&(i, j)) {
//                     claim_names.insert((i, j), r[0]);
//                 } else {
//                     intersecting.insert(claim_names[&(i, j)]);
//                     intersecting.insert(r[0]);
//                 }
//             }
//         }
//     }
//     let out1 = claims.values().filter(|v| **v > 1).count();
//     println!("I: {}", out1);
//     let out2 = all.difference(&intersecting).next();
//     println!("II: {:?}", out2);
//}
