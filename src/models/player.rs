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
    dir : String,
    idle: bool,
    walk_count: f64,
    prev_dir: String,
    speed: f64,
    controller: Controller,

}

impl Player {
    /// Create a new `Player` with a random position and direction
    pub fn new(id: i32, point: Point, controller: Controller) -> Self {
        let speed: f64 = 80.0;
        let img: &str;
        if (id ==1) {
            img = "image/akane.png";
        } else {
            img = "image/aoi.png";
        }
        Player {
            id: id,
            alive: true,
            point: point,
            dir : String::from("down"),
            prev_dir: String::from("down"),
            idle: true,
            walk_count: 0.0,
            speed: speed,
            controller: controller,

        }
    }
    pub fn update(&mut self, dt: f64, actions: &HashMap<String, bool>, event: &mut Vec<EventType>){
        let mut x = 0.0;
        let mut y = 0.0;


        if actions.get(&self.controller.Up) == Some(&true) {
            y -= dt * self.speed;
        }

        if actions.get(&self.controller.Down) == Some(&true) {
            y += dt * self.speed;
        }

        if actions.get(&self.controller.Right) == Some(&true) {
            x += dt * self.speed;
        }

        if actions.get(&self.controller.Left) == Some(&true) {
            x -= dt * self.speed;
        }
        self.point.y += y;
        self.point.x += x;
        if (y < 0.0){
            self.idle=false;
            self.dir = "up".to_string();
        }else if (y > 0.0){
            self.idle=false;
            self.dir = "down".to_string();
        }else if (x > 0.0){
            self.idle=false;
            self.dir = "right".to_string();
        }else if (x < 0.0){
            self.idle=false;
            self.dir = "left".to_string();
        }else{
            self.idle=true;
        }

        if (&self.prev_dir == &self.dir){
        }else {
            self.prev_dir =  String::from(&self.dir.clone());
        }




        if actions.get(&self.controller.A) == Some(&true) {
            let new_bomb = Bomb::new(self.point);
            event.push(EventType::SetBomb(Bomb::new(self.point)));
        }

    }
    pub fn draw(&mut self){

        let mut y = 0;
        let mut x = 0;

        if (self.idle == true) {self.walk_count = 0.0;}
        else {self.walk_count += self.speed;}
        //log(&self.walk_count.to_string());

        // draw_player(self.point.x, self.point.y);
        if (self.dir == "up"){y=2}
        if (self.dir == "down"){y=0}
        if (self.dir == "left"){y=1}
        if (self.dir == "right"){y=3}

        if (self.walk_count > 0.0) {x=2}
        if (self.walk_count > 800.0) {x=1}
        if (self.walk_count > 1600.0) {x=0}
        if (self.walk_count > 2400.0) {x=1; self.walk_count=-9.0;}

        let id = "player".to_string() + &self.id.to_string();
        draw_player_animation(&id, x, y, self.point.x, self.point.y); 
//        self.sprite.animate(self.point.x, self.point.y, &self.dir, self.idle, self.walking); 
    }
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/javascript/canvas.js")]
extern "C" {
    pub fn draw_player(x: f64, y: f64);
    pub fn draw_player_animation(id: &str,recX:i32, recY:i32, x: f64, y: f64);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
