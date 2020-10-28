use crate::geometry::{Point, Size};
use crate::controller::{Actions, Controller, Event, EventType};
use crate::models::Bomb;
use crate::models::Wall;
use std::collections::HashMap;
//use crate::controller::Actions;

/// The `Player` is the rocket controlled by the user
//#[derive(Default)]
pub struct Player {
    id: i32,
    alive: bool,
    point: Point,
    speed: f64,
    radius: f64,
    controller: Controller,

}

impl Player {
    /// Create a new `Player` with a random position and direction
    pub fn new(id: i32, point: Point, controller: Controller) -> Self {
        let speed: f64 = 80.0;
        let radius: f64 = 20.0;
        Player {
            id: 100,
            alive: true,
            point: point,
            speed: speed,
            radius: radius,
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

    pub fn collide_with_bomb(&self,obj: &Bomb) -> i32 {
        let radii = self.radius + obj.radius;
        if (self.point.squared_distance_to(&obj.point) < radii * radii){
            // let testaaa = self.id/100*10+obj.id/100;
            // alert(&testaaa.to_string());
            return (self.id / 100 * 10 + obj.id / 100); //playerid 10X, bombid 20X, wallid 30X
        }
        else {
            return 0;
        }
    }

    pub fn collide_with_wall(&self,obj: &Wall) -> i32 {
        let radii = self.radius + obj.radius;
        if (self.point.squared_distance_to(&obj.point) < radii * radii){
            return (self.id / 100 * 10 + obj.id / 100); //playerid 10X, bombid 20X, wallid 30X
        }
        else {
            return 0;
        }
    }

    pub fn relocate(&mut self, dt: f64, actions: &HashMap<String, bool>) {
        //xとy，objとの差が大きいほうが衝突値
        //衝突値の差分をリサイズする
        if actions.get(&self.controller.Up) == Some(&true) {
            self.point.y += dt * self.speed;
        }

        if actions.get(&self.controller.Down) == Some(&true) {
            self.point.y -= dt * self.speed;
        }

        if actions.get(&self.controller.Right) == Some(&true) {
            self.point.x -= dt * self.speed;
        }

        if actions.get(&self.controller.Left) == Some(&true) {
            self.point.x += dt * self.speed;
        }
    }
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/javascript/canvas.js")]
extern "C" {
    pub fn draw_player(x: f64, y: f64);
}
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}
