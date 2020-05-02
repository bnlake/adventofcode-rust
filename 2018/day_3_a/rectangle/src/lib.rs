use point::Point;

pub struct Rectangle {
    pub id: i32,
    pub p1: Point,
    pub p2: Point,
}

impl Rectangle {
    pub fn new(id: i32, p1: Point, p2: Point) -> Self {
        Rectangle { id, p1, p2 }
    }
}

impl Rectangle {
    pub fn left(&self) -> i32 {
        self.p1.x
    }
    pub fn right(&self) -> i32 {
        self.p2.x
    }
    pub fn top(&self) -> i32 {
        self.p1.y
    }
    pub fn bottom(&self) -> i32 {
        self.p2.y
    }

    pub fn width(&self) -> i32 {
        self.right() - self.left()
    }
    pub fn height(&self) -> i32 {
        self.bottom() - self.top()
    }

    /// # Examples
    /// ```rust
    /// use rectangle::Rectangle;
    /// use point::Point;
    ///
    /// let x = Rectangle::new(1, Point::new(0,0), Point::new(2, 4));
    /// assert_eq!(x.area(), 8);
    /// ```
    pub fn area(&self) -> i32 {
        self.width() * self.height()
    }
}

/// # Examples
///
/// ```rust
/// use rectangle::Rectangle;
/// use point::Point;
///
/// let x: Rectangle = Rectangle::new(1, Point::new(1, 1), Point::new(2, 2));
/// let y: Rectangle = Rectangle::new(2, Point::new(1, 1), Point::new(2, 2));
///
/// assert!(&x == &y);
/// ```
impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        self.p1 == other.p1 && self.p2 == other.p2
    }
}