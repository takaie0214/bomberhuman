use crate::geometry::{Point, Size};
use crate::controller::{Actions, Controller};
use std::collections::HashMap;
//use crate::controller::Actions;

/// The `Player` is the rocket controlled by the user
//#[derive(Default)]
pub struct Player {
    id: i32,
    alive: bool,
    point: Point,
    speed: f64,
    controller: Controller,

}

impl Player {
    /// Create a new `Player` with a random position and direction
    pub fn new(id: i32, point: Point, controller: Controller) -> Self {
        let speed: f64 = 80.0;
        Player {
            id: id,
            alive: true,
            point: point,
            speed: speed,
            controller: controller,
        }
    }
    pub fn update(&mut self, dt: f64, actions: &HashMap<String, bool>){

        if actions.get(&self.controller.Up) == Some(&true) {
            *self.y_mut() -= dt * self.speed;
        }

        if actions.get(&self.controller.Down) == Some(&true) {
            *self.y_mut() += dt * self.speed;
        }

        if actions.get(&self.controller.Right) == Some(&true) {
            *self.x_mut() += dt * self.speed;
        }

        if actions.get(&self.controller.Left) == Some(&true) {
            *self.x_mut() -= dt * self.speed;
        }
    }
    pub fn x(&self) -> f64 {
        self.point.x
    }
    pub fn y(&self) -> f64 {
        self.point.y
    }
    pub fn x_mut(&mut self) -> &mut f64 {
        &mut self.point.x
    }
    pub fn y_mut(&mut self) -> &mut f64 {
        &mut self.point.y
    }

}

