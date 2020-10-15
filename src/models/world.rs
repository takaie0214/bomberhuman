use crate::geometry::{Point, Size};
use crate::models::Player;
use crate::controller::{Actions, Controller};
use std::collections::HashMap;

/// A model that contains the other models and renders them
pub struct World {
    pub players: Vec<Player>,
    pub size: Size
}

impl World {
    /// Returns a new world of the given size
    pub fn new(size: Size) -> World {

        let mut point = Point::new(20.0, 50.0);
        let id = 1;
        let controller = Controller::new("ArrowUp","ArrowDown","ArrowLeft","ArrowRight");
        let mut players = Vec::new();

        players.push(Player::new(id, point, controller));
        World {
            players: players,
            size: size
        }
    }

    pub fn update(&mut self, dt: f64, actions: &HashMap<String, bool>) {
        for player in &mut self.players {
            player.update(dt, actions);
        }
    }
}
