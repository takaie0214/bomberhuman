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

        let mut players = Vec::new();

        let id1 = 1;
        let mut point1 = Point::new(20.0, 50.0);
        let controller1 = Controller::new("ArrowUp","ArrowDown","ArrowRight","ArrowLeft");
        players.push(Player::new(id1, point1, controller1));

        let id2 = 2;
        let mut point2 = Point::new(150.0, 150.0);
        let controller2 = Controller::new("w","s","d","a");
        players.push(Player::new(id2, point2, controller2));

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
