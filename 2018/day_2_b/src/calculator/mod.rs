// Main algorithm, takes raw data and solves the challenge
pub fn find_common_characters_of_proper_box(contents: String) -> String {
    let unique_box_strings: Vec<String> = parse_contents_to_vect(contents);
    let mut result: String = String::new();

    // With collection, loop through each row in set
    // Labeling the for loops means we can exit the parent loop
    // from inside a nested loop and avoid unnecessary iterations.
    'outer: for i in 0..unique_box_strings.len() {
        if i >= unique_box_strings.len() - 1 {
            break 'outer;
        } else {
            'inner: for j in i + 1..unique_box_strings.len() {
                if strings_meet_criteria(&unique_box_strings[i], &unique_box_strings[j]) {
                    // Store the extracted characters that match
                    result = String::from_utf8(extract_same_chars_from_two_strings(
                        &unique_box_strings[i],
                        &unique_box_strings[j],
                    ))
                    .unwrap();
                    break 'outer;
                }
            }
        }
    }
    return result;
}


// Given a string with new line characters: converts the lines into a collection
fn parse_contents_to_vect(contents: String) -> Vec<String> {
    let mut x: Vec<String> = vec![];
    for line in contents.lines() {
        x.push(String::from(line));
    }
    x
}


// Given two strings: assert that the strings have identical characters at each
// index other than 1
fn strings_meet_criteria(x: &str, y: &str) -> bool {
    let x = x.as_bytes();
    let y = y.as_bytes();

    if x.len() != y.len() {
        false
    } else {
        let mut wrong_count: u8 = 0;
        for i in 0..x.len() {
            if x[i] != y[i] {
                wrong_count = wrong_count + 1;
            }
        }
        wrong_count == 1
    }
}


// Given two strings: extract the characters that match in the same index locations
fn extract_same_chars_from_two_strings(x: &str, y: &str) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    let x = x.as_bytes();
    let y = y.as_bytes();

    for i in 0..x.len() {
        match x[i] == y[i] {
            true => result.push(x[i]),
            false => (),
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_common_characters_of_proper_box() {
        let contents = String::from(
            "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz",
        );
        let control: String = String::from("fgij");

        assert_eq!(find_common_characters_of_proper_box(contents), control);
    }

    #[test]
    fn parses_contents_into_hashset() {
        let contents = String::from("abcde");
        let mut control: Vec<String> = vec![];
        control.push(String::from("abcde"));

        parse_contents_to_vect(contents);
    }

    #[test]
    fn checking_strings_meet_criteria() {
        let a: String = String::from("fghij");
        let mut b: String = String::from("fguij");
        assert_eq!(strings_meet_criteria(&a, &b), true);

        b = String::from("fgujj");
        assert_ne!(strings_meet_criteria(&a, &b), true);
    }

    #[test]
    fn extracts_correct_characters() {
        let a: String = String::from("fghij");
        let b: String = String::from("fguij");
        let control: String = String::from("fgij");

        let result: String =
            String::from_utf8(extract_same_chars_from_two_strings(&a, &b)).unwrap();

        assert_eq!(result, control);
    }
}
