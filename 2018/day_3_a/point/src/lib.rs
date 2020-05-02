pub struct Point {
    pub x: i32,
    pub y: i32,
}


impl Point {
    /// Generates a new Point struct and transfers ownership
    ///
    /// # Examples
    ///
    /// ```rust
    /// use point::Point;
    /// let test = Point::new(3,2);
    /// let control = Point{x: 3, y: 2};
    ///
    /// assert_eq!(test == control, true);
    /// ```
    pub fn new(x: i32, y: i32) -> Self {
        Point {
            x,
            y,
        }
    }
}

/// Checks if two points contain identical `x` and `y` values
///
/// # Examples
///
/// ```rust
/// use point::Point;
/// let x = Point { x: 1, y: 1 };
/// let y = Point { x: 1, y: 1 };
/// let z = Point { x: 1, y: 2 };
///
/// assert_eq!(x == y, true);
/// assert_eq!(x != z, true);
/// ```
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}