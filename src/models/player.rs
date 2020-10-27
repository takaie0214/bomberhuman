use crate::geometry::{Point, Size};
use crate::controller::{Actions, Controller, Event, EventType};
use crate::models::Bomb;
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
    pub fn update(&mut self, dt: f64, actions: &HashMap<String, bool>, event: &mut Vec<EventType>){

        if actions.get(&self.controller.Up) == Some(&true) {
            self.point.y -= dt * self.speed;
        }

        if actions.get(&self.controller.Down) == Some(&true) {
            self.point.y += dt * self.speed;
        }

        if actions.get(&self.controller.Right) == Some(&true) {
            self.point.x += dt * self.speed;
        }

        if actions.get(&self.controller.Left) == Some(&true) {
            self.point.x -= dt * self.speed;
        }
        if actions.get(&self.controller.A) == Some(&true) {
            let new_bomb = Bomb::new(self.point);
            event.push(EventType::SetBomb(Bomb::new(self.point)));
        }

    }
    pub fn draw(&self){
        draw_player(self.point.x, self.point.y);
    }
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/javascript/canvas.js")]
extern "C" {
    pub fn draw_player(x: f64, y: f64);
}
