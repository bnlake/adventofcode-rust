use std::cmp::Ordering;

pub struct Timestamp {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
}

impl Timestamp {
    pub fn from(raw_timestamp: &str) -> Timestamp {
        let x: Vec<i32> = raw_timestamp
            .split(|x| x == '-' || x == ' ' || x == ':')
            .filter_map(|x| x.parse().ok())
            .collect();
        return if let [year, month, day, hour, minute] = x[..] {
            Timestamp {
                year,
                month,
                day,
                hour,
                minute,
            }
        } else {
            Timestamp {
                year: 0,
                month: 0,
                day: 0,
                hour: 0,
                minute: 0,
            }
        };
    }
}

/// #Example
/// ```rust
/// use timestamp::Timestamp;
/// let x = Timestamp::from("1518-08-30 00:49");
/// let y = Timestamp::from("1518-08-30 00:49");
///
/// assert!(x == y);
/// ```
impl PartialEq for Timestamp {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year
            && self.month == other.month
            && self.day == other.day
            && self.hour == other.hour
            && self.minute == other.minute
    }
}

impl PartialOrd for Timestamp {
    /// #Example
    /// ```rust
    /// use timestamp::Timestamp;
    /// use std::cmp::Ordering;
    /// let x = Timestamp::from("1518-08-30 00:48");
    /// let y = Timestamp::from("1518-08-30 00:49");
    ///
    /// assert_eq!(x.partial_cmp(&y), Some(Ordering::Less));
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self < other {
            Some(Ordering::Less)
        } else if self > other {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }

    /// #Example
    /// ```rust
    /// use timestamp::Timestamp;
    /// let x = Timestamp::from("1518-08-30 00:48");
    /// let y = Timestamp::from("1518-08-30 00:49");
    ///
    /// assert_eq!(x < y, true);
    /// assert_eq!(y < x, false);
    /// ```
    fn lt(&self, other: &Self) -> bool {
        self.year < other.year
            || self.month < other.month
            || self.day < other.day
            || self.hour < other.hour
            || self.minute < other.minute
    }

    /// #Example
    /// ```rust
    /// use timestamp::Timestamp;
    /// let x = Timestamp::from("1518-08-29 00:49");
    /// let y = Timestamp::from("1518-08-30 00:49");
    ///
    /// assert!(x <= y);
    /// ```
    fn le(&self, other: &Self) -> bool {
        self == other
            || !(self.year > other.year)
            || !(self.month > other.month)
            || !(self.day > other.day)
            || !(self.hour > other.hour)
            || !(self.minute > other.minute)
    }

    /// #Example
    /// ```rust
    /// use timestamp::Timestamp;
    /// let x = Timestamp::from("1518-08-30 23:48");
    /// let y = Timestamp::from("1518-08-30 00:49");
    ///
    /// assert_eq!(x > y, true);
    /// assert_eq!(y > x, false);
    /// ```
    fn gt(&self, other: &Self) -> bool {
        self.year > other.year
            || self.month > other.month
            || self.day > other.day
            || self.hour > other.hour
            || self.minute > other.minute
    }

    fn ge(&self, other: &Self) -> bool {
        !(self.year < other.year)
            || !(self.month < other.month)
            || !(self.day < other.day)
            || !(self.hour < other.hour)
            || !(self.minute < other.minute)
    }
}
