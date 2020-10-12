use crate::geometry::{Point, Size};
use crate::models::Player;
use crate::controller::Actions;

/// A model that contains the other models and renders them
pub struct World {
    pub player: Player,
    pub size: Size
}

impl World {
    /// Returns a new world of the given size
    pub fn new(size: Size) -> World {
        let radius: i32 = 3;
        let a: f64 = 50.0;
        let b: f64 = 20.0;
        let mut point = Point::new(a, b);
        World {
            player: Player::new(point),
            size: size
        }
    }
    pub fn update(&mut self, dt: f64, actions: &Actions) {
        self.player.update(dt, actions);
    }
}
