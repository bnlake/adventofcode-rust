use std::collections::HashSet;
use std::ops::Deref;

pub fn find_first_repeating_frequency_result(frequency_change_seq: &Vec<i32>) -> i32 {
    // Store the unique values of frequency results
    let mut frequency_results: HashSet<i32> = HashSet::new();
    // We need to keep track of the iterative result since we may loop through multiple times
    let mut running_result: i32 = 0;

    let mut result: i32 = 0;
    let mut result_found: bool = false;

    // Use rust's loop like a do while
    loop {
        for freq_change in frequency_change_seq.clone() {
            // Calculate the result of next freq change
            running_result = running_result + freq_change;

            // Identify if new result already exists in the set of frequency results
            // If it was found then we have our answer
            if frequency_results.contains(&running_result) {
                result = running_result;
                result_found = true;
                break;
            } else {
                frequency_results.insert(running_result);
            }
        }

        if result_found {
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_first_repeating_freq_result() {
        let set: Vec<i32> = vec![3, 3, 4, -2, -4];
        let control: i32 = 10;

        assert_eq!(find_first_repeating_frequency_result(&set), control);
    }
}