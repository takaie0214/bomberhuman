//use rand::Rng;

//use super::Point;

/// A `Size` represents a region in space
#[derive(Clone, Copy, Default)]
pub struct Size {
    pub width: i32,
    pub height: i32
}

impl Size {
    /// Returns a new `Size` of the given dimensions
    pub fn new(width: i32, height: i32) -> Size {
        Size { width: width, height: height }
    }

//    /// Returns true if the `Point` is contained in this `Size` or false otherwise
//    pub fn contains(&self, point: Point) -> bool {
//        0.0 <= point.x && point.x <= self.width
//            && 0.0 <= point.y && point.y <= self.height
//    }
//
//    /// Returns a random x coordinate within the bounds of this `Size`
//    pub fn random_x<R: Rng>(&self, rng: &mut R) -> f64 {
//        rng.gen_range(0.0, self.width)
//    }
//
//    /// Returns a random y coordinate within the bounds of this `Size`
//    pub fn random_y<R: Rng>(&self, rng: &mut R) -> f64 {
//        rng.gen_range(0.0, self.height)
//    }
}
