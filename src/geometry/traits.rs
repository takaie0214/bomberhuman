use super::{Point, Size};

/// A trait for objects that occupy a position in space
pub trait Position {
    /// Returns the x coordinate of the object
    fn x(&self) -> i32;

    /// Returns the y coordinate of the object
    fn y(&self) -> i32;

}

