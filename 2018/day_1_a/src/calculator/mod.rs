pub fn calculate_final_value(set: Vec<i32>) -> i32 {
    let mut finalvalue: i32 = 0;

    for i in 0..set.len() {
        finalvalue = finalvalue + set[i];
    }

    finalvalue
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::calculator::calculate_final_value;

    #[test]
    fn returns_final_val_from_vect() {
        let set: Vec<i32> = vec![1, 1, -2];
        let control: i32 = 0;

        assert_eq!(calculate_final_value(set), control);
    }
}