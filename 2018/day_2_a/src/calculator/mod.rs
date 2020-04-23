use std::collections::HashSet;

struct BoxIdChecksum {
    has_a_two: bool,
    has_a_three: bool,
}

pub fn calculate_checksum(contents: &String) -> i32 {
    let mut count_three: i32 = 0;
    let mut count_two: i32 = 0;

    for line in contents.lines() {
        let result = process_box_id(line);
        // Let's add to the total count of matches
        if result.has_a_two {
            count_two = count_two + 1;
        }

        if result.has_a_three {
            count_three = count_three + 1;
        }
    }

    return count_three * count_two;
}

fn process_box_id(x: &str) -> BoxIdChecksum {
    let mut box_id_checksum = BoxIdChecksum {
        has_a_two: false,
        has_a_three: false,
    };

    let all_chars: Vec<u8> = Vec::from(x.as_bytes());

    // In an effort to reduce the times we iterate the vec
    // we'll get a hashset of the values in the vec so we can
    // remove values that we've already found
    let mut unique_chars: HashSet<u8> = HashSet::new();
    for char in &all_chars {
        unique_chars.insert(*char);
    }

    // Let's get the count of each unique char in all chars
    // and use an enum
    for char in unique_chars {
        let count = all_chars.iter().filter(|&x| *x == char).count();
        if count == 2 {
            box_id_checksum.has_a_two = true;
        } else if count == 3 {
            box_id_checksum.has_a_three = true;
        }
    }
    box_id_checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_checksum() {
        let contents = String::from(
            "abcdef\
bababc
abbcde
abcccd
aabcdd
abcdee
ababab",
        );
        let control: i32 = 12;

        assert_eq!(calculate_checksum(&contents), control);
    }
}
