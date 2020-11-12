use crate::geometry::{Point, Size, Position};
use crate::controller::{Actions, Controller, Event, EventType};
use crate::models::{Bomb, Wall, Block, Fire};
use std::collections::HashMap;
//use crate::controller::Actions;

/// The `Player` is the rocket controlled by the user
//#[derive(Default)]
pub struct Player {
    id: i32,
    alive: bool,
    on_bomb: [bool; 5],//Vecが望まし
    bomb_count: i32,
    point: Point,
    dir : String,
    idle: bool,
    walk_count: i32,
    prev_dir: String,
    speed: f64,
    radius: i32,
    controller: Controller,

}

impl Player {
    /// Create a new `Player` with a random position and direction
    pub fn new(id: i32, point: Point, controller: Controller) -> Self {
        let speed: f64 = 80.0;
        let radius: i32 = 24;
        let on_bomb: [bool; 5] = [false,false,false,false,false];

        Player {
            id: id,
            alive: true,
            on_bomb: on_bomb,
            bomb_count: 0,
            point: point,
            dir : String::from("down"),
            prev_dir: String::from("down"),
            idle: true,
            walk_count: 0,
            speed: speed,
            radius: radius,
            controller: controller,

        }
    }
    pub fn update(&mut self, dt: f64, actions: &HashMap<String, bool>, event: &mut Vec<EventType>){
        let mut x = 0;
        let mut y = 0;


        if actions.get(&self.controller.Up) == Some(&true) {
            y -= (dt * self.speed) as i32;
        }

        if actions.get(&self.controller.Down) == Some(&true) {
            y += (dt * self.speed) as i32;
        }

        if actions.get(&self.controller.Right) == Some(&true) {
            x += (dt * self.speed) as i32;
        }

        if actions.get(&self.controller.Left) == Some(&true) {
            x -= (dt * self.speed) as i32;
        }
        self.point.y += y;
        self.point.x += x;
        if (y < 0){
            self.idle=false;
            self.dir = "up".to_string();
        }else if (y > 0){
            self.idle=false;
            self.dir = "down".to_string();
        }else if (x > 0){
            self.idle=false;
            self.dir = "right".to_string();
        }else if (x < 0){
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
            self.on_bomb[(self.bomb_count) as usize]=true;

         //   let new_bomb = Bomb::new(200+self.id%10*10+self.bomb_count,((self.point.x) as i32 / 50*50 + 25)as f64,((self.point.y) as i32 /50*50 + 25) as f64);
        let x = self.point.x  / 50*50 + 25;
        let y = self.point.y  /50*50 + 25;
        let id = 200+self.id%10*10+self.bomb_count;

            event.push(EventType::SetBomb{id,x,y});
            self.bomb_count += 1;
            self.bomb_count %= 5;
        }

    }
    pub fn draw(&mut self){

        let mut y = 0;
        let mut x = 0;

        if (self.idle == true) {self.walk_count = 0;}
        else {self.walk_count += self.speed as i32;}
        //log(&self.walk_count.to_string());

        // draw_player(self.point.x, self.point.y);
        if (self.dir == "up"){y=2}
        if (self.dir == "down"){y=0}
        if (self.dir == "left"){y=1}
        if (self.dir == "right"){y=3}

        if (self.walk_count > 0) {x=2}
        if (self.walk_count > 800) {x=1}
        if (self.walk_count > 1600) {x=0}
        if (self.walk_count > 2400) {x=1; self.walk_count=-9;}

        let id = "player".to_string() + &self.id.to_string();
        draw_player_animation(&id, x, y, self.point.x, self.point.y); 
//        self.sprite.animate(self.point.x, self.point.y, &self.dir, self.idle, self.walking); 
    }

    pub fn collide_with_bomb(&mut self,obj: &Bomb) -> i32 {
        let radii = self.radius + obj.radius;
        // if (((self.point.x - obj.point.x).abs() < 47.0) & ((self.point.y - obj.point.y).abs() < 47.0)){
        //     return 0;
        // }
        if (((self.point.x - obj.point.x).abs() < radii) & ((self.point.y - obj.point.y).abs() < radii)){
            if ((self.id%10 == obj.id%100/10) & (self.on_bomb[(obj.id%10) as usize])){
                // let test = self.id;
                // let testa = obj.id;
                // log(&test.to_string());
                // log(&testa.to_string());
                return 0;
            }
            else {
                return (self.id / 100 * 10 + obj.id / 100);

            }
        }
        else {
            if (self.id%10 == obj.id%100/10){
                self.on_bomb[(obj.id%10) as usize] = false;
            }
            return 0;
        }
        // if (self.point.squared_distance_to(&obj.point) < radii * radii){
        //     // let testaaa = self.id/100*10+obj.id/100;
        //     // alert(&testaaa.to_string());
        //     return (self.id / 100 * 10 + obj.id / 100); //playerid 10X, bombid 20X, wallid 30X
        // }
        // else {
        //     return 0;
        // }
    }

    pub fn collide_with_wall(&self,obj: &Wall) -> i32 {
        let radii = self.radius + obj.radius;
        if (((self.point.x - obj.point.x).abs() < radii) & ((self.point.y - obj.point.y).abs() < radii)){
            return (self.id / 100 * 10 + obj.id / 100);
        }
        else {
            return 0;
        }
        // if (self.point.squared_distance_to(&obj.point) < radii * radii){
        //     return (self.id / 100 * 10 + obj.id / 100); //playerid 10X, bombid 20X, wallid 30X
        // }
        // else {
        //     return 0;
        // }
    }
    pub fn collide_with_block(&self,obj: &Block) -> i32 {
        let radii = self.radius + obj.radius;
        if (((self.point.x - obj.point.x).abs() < radii) & ((self.point.y - obj.point.y).abs() < radii)){
            return (self.id / 100 * 10 + obj.id / 100);
        }
        else {
            return 0;
        }
        // if (self.point.squared_distance_to(&obj.point) < radii * radii){
        //     return (self.id / 100 * 10 + obj.id / 100); //playerid 10X, bombid 20X, wallid 30X
        // }
        // else {
        //     return 0;
        // }
    }
    pub fn collide_with_fire(&self,obj: &Fire) -> i32 {
        let radii = self.radius + obj.radius;
        if (((self.point.x - obj.point.x).abs() < radii) & ((self.point.y - obj.point.y).abs() < radii)){
            return (self.id / 100 * 10 + obj.id / 100);
        }
        else {
            return 0;
        }
        // if (self.point.squared_distance_to(&obj.point) < radii * radii){
        //     return (self.id / 100 * 10 + obj.id / 100); //playerid 10X, bombid 20X, wallid 30X
        // }
        // else {
        //     return 0;
        // }
    }
    pub fn die(&mut self) {
        self.alive = false;
    }


    pub fn relocate(&mut self, dt: f64, actions: &HashMap<String, bool>, obj_point: &Point) {
        if actions.get(&self.controller.Up) == Some(&true) {
            self.point.y += (dt * self.speed) as i32;
        }

        if actions.get(&self.controller.Down) == Some(&true) {
            self.point.y -= (dt * self.speed) as i32;
        }

        if actions.get(&self.controller.Right) == Some(&true) {
            self.point.x -= (dt * self.speed) as i32;
        }

        if actions.get(&self.controller.Left) == Some(&true) {
            self.point.x += (dt * self.speed) as i32;
        }
        if (self.point.x - obj_point.x > self.radius) & ((actions.get(&self.controller.Up) == Some(&true)) | (actions.get(&self.controller.Down) == Some(&true))){
            self.point.x += (dt * self.speed) as i32;
        }
        if (obj_point.x - self.point.x > self.radius) & ((actions.get(&self.controller.Up) == Some(&true)) | (actions.get(&self.controller.Down) == Some(&true))){
            self.point.x -= (dt * self.speed) as i32;
        }
        if (self.point.y - obj_point.y > self.radius) & ((actions.get(&self.controller.Left) == Some(&true)) | (actions.get(&self.controller.Right) == Some(&true))){
            self.point.y += (dt * self.speed) as i32;
        }
        if (obj_point.y - self.point.y > self.radius) & ((actions.get(&self.controller.Left) == Some(&true))| (actions.get(&self.controller.Right) == Some(&true))){
            self.point.y -= (dt * self.speed) as i32;
        }
    }
}


impl Position for Player{
    fn x(&self) -> i32{
        self.point.x

    }
    fn y(&self) -> i32{
        self.point.y

    }

}

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/javascript/canvas.js")]
extern "C" {
    pub fn draw_player(x: i32, y: i32);
    pub fn draw_player_animation(id: &str,recX:i32, recY:i32, x: i32, y: i32);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}
