use std::collections::HashMap;

pub struct Password(pub [u8; 6]);

impl From<i32> for Password {
    fn from(input: i32) -> Self {
        let d0 = ((input / 100_000) % 10) as u8;
        let d1 = ((input / 10_000) % 10) as u8;
        let d2 = ((input / 1_000) % 10) as u8;
        let d3 = ((input / 100) % 10) as u8;
        let d4 = ((input / 10) % 10) as u8;
        let d5 = (input % 10) as u8;

        Self([d0, d1, d2, d3, d4, d5])
    }
}

impl Password {
    /// ```rust
    /// use password::Password;
    /// let x = Password::from(122345 as i32);
    /// assert!(x.has_identical_adjacent_digits());
    /// ```
    pub fn has_identical_adjacent_digits(&self) -> bool {
        (0..self.0.len() - 1).any(|x| self.0[x] == self.0[x + 1])
    }

    /// ```rust
    /// use password::Password;
    /// let w = Password::from(122234 as i32);
    /// let x = Password::from(112233 as i32);
    /// let y = Password::from(123444 as i32);
    /// let z = Password::from(111122 as i32);
    ///
    /// assert!(!w.has_identical_adjacent_digits_in_pairs());
    /// assert!(x.has_identical_adjacent_digits_in_pairs());
    /// assert!(!y.has_identical_adjacent_digits_in_pairs());
    /// assert!(z.has_identical_adjacent_digits_in_pairs());
    /// ```
    pub fn has_identical_adjacent_digits_in_pairs(&self) -> bool {
        let mut map: HashMap<u8, u8> = HashMap::new();

        for i in 0..self.0.len() {
            if map.contains_key(&self.0[i]) {
                *map.get_mut(&self.0[i]).unwrap() += 1;
            } else {
                for (k, v) in map.drain() {
                    if v > 2 && (v % 2) == 1 {
                        return false;
                    }
                }
                map.insert(self.0[i], 1);
            }
        }
        for (k, v) in map.drain() {
            if v > 2 && (v % 2) == 1 {
                return false;
            }
        }
        return true;
    }

    /// ```rust
    /// use password::Password;
    /// let x = Password::from(123456 as i32);
    /// assert!(x.is_increasing_each_digit());
    /// ```
    pub fn is_increasing_each_digit(&self) -> bool {
        (0..self.0.len() - 1).all(|i| self.0[i] <= self.0[i + 1])
    }
}
