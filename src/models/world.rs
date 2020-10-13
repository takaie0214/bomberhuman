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
        let speed: f64 = 20.0;
        let mut point = Point::new(20.0, 50.0);
        World {
            player: Player::new(point, speed),
            size: size
        }
    }
    pub fn update(&mut self, dt: f64, actions: &Actions) {
        self.player.update(dt, actions);
    }
}
