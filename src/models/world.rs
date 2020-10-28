use crate::geometry::{Point, Size};
use crate::models::{Bomb, Player,Wall};
use std::collections::HashMap;
use crate::controller::{Actions, Controller, Event, EventType};

/// A model that contains the other models and renders them
pub struct World {
    pub players: Vec<Player>,
    pub bomb: Vec<Bomb>,
    pub walls: Vec<Wall>,
    pub size: Size,
    pub event: Vec<EventType>,
}

impl World {
    pub fn new(size: Size) -> World {

        let mut players = Vec::new();

        let id1 = 1;
        let mut point1 = Point::new(75.0, 75.0);
        let controller1 = Controller::new("ArrowUp","ArrowDown","ArrowRight","ArrowLeft", "Space");
        players.push(Player::new(id1, point1, controller1));

        let id2 = 2;
        let mut point2 = Point::new(150.0, 150.0);
        let controller2 = Controller::new("w","s","d","a", "m");
        players.push(Player::new(id2, point2, controller2));

        let wall_bmp = [[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
                       [1,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                       [1,0,1,0,1,0,1,0,1,0,1,0,1,0,1],
                       [1,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                       [1,0,1,0,1,0,1,0,1,0,1,0,1,0,1],
                       [1,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                       [1,0,1,0,1,0,1,0,1,0,1,0,1,0,1],
                       [1,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                       [1,0,1,0,1,0,1,0,1,0,1,0,1,0,1],
                       [1,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                       [1,0,1,0,1,0,1,0,1,0,1,0,1,0,1],
                       [1,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                       [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]];

        let mut walls = Vec::new();
        for (index1, val1) in wall_bmp.iter().enumerate() {
            for (index2, val2)in val1.iter().enumerate() {
                if (*val2 == 1) {
                    walls.push(Wall::new(Point::new(50.0 * index2 as f64+25.0, 50.0 * index1 as f64+25.0)));
                }
            }
        }


        World {
            players: players,
            bomb: vec![],
            walls: walls,
            size: size,
            event: vec![],
        }
    }

    pub fn update(&mut self, dt: f64, actions: &HashMap<String, bool>) {
        for p in &mut self.players {
            p.update(dt, actions, &mut self.event);
        }
        for b in &mut self.bomb {
            b.update(dt,  &mut self.event);
        }
        match self.event.pop() {
            None => (),
            Some(T) =>
                match T {
                    EventType::SetBomb(bomb) => self.bomb.push(bomb),
                    EventType::Explosion => (),
                }
        }
    }
    pub fn draw(&self){
        for p in &self.players {
            p.draw();
        }
        for w in &self.walls {
            w.draw();
        }
        for b in &self.bomb {
            b.draw();
        }
    }
}

