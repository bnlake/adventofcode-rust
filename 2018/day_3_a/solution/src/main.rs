use parser;
use std::collections::HashMap;
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
        Ok(area) => println!("Total overlapping area is {} squared", area),
        Err(e) => println!("Couldn't calculate area: {}", e),
    }

    Ok(())
}

pub fn find_total_square_inches_of_overlapping_claims(
    raw_claims: Vec<&str>,
) -> Result<i32, Box<dyn error::Error>> {
    let mut claims_map: HashMap<(i32, i32), i32> = HashMap::new();

    for raw_claim in raw_claims.iter() {
        let temp: Vec<i32> = raw_claim
            .split(|x| x == ' ' || x == '@' || x == ',' || x == ':' || x == 'x' || x == '#')
            .filter_map(|x| x.parse().ok())
            .collect();
        if let [left, top, width, height] = temp[1..] {
            for x in left..left + width {
                for y in top..top + height {
                    // We're going to add each coordinate (unit = inches) to a hashmap
                    // and increase it's value if it was already entered
                    *claims_map.entry((x, y)).or_insert(0) += 1;
                }
            }
        }
    }
    // Count every coordinate that has a value greater than 1
    // (meaning 2 or more claims cover that coordinate)
    let result: i32 = claims_map.values().filter(|&&x| x > 1).count() as i32;
    Ok(result)
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
