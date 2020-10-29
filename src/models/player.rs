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
    on_bomb: [bool; 5],//Vecが望まし
    bomb_count: i32,
    point: Point,
    speed: f64,
    radius: f64,
    controller: Controller,

}

impl Player {
    /// Create a new `Player` with a random position and direction
    pub fn new(id: i32, point: Point, controller: Controller) -> Self {
        let speed: f64 = 80.0;
        let radius: f64 = 24.0;
        let on_bomb: [bool; 5] = [false,false,false,false,false];
        Player {
            id: id,
            alive: true,
            on_bomb: on_bomb,
            bomb_count: 0,
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
            self.on_bomb[(self.bomb_count) as usize]=true;

         //   let new_bomb = Bomb::new(200+self.id%10*10+self.bomb_count,((self.point.x) as i32 / 50*50 + 25)as f64,((self.point.y) as i32 /50*50 + 25) as f64);
        let x = ((self.point.x) as i32 / 50*50 + 25)as f64;
        let y = ((self.point.y) as i32 /50*50 + 25) as f64;
        let id = 200+self.id%10*10+self.bomb_count;

            event.push(EventType::SetBomb{id,x,y});
            self.bomb_count += 1;
            self.bomb_count %= 5;
        }

    }
    pub fn draw(&self){
        draw_player(self.point.x, self.point.y);
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

    pub fn relocate(&mut self, dt: f64, actions: &HashMap<String, bool>, obj_point: &Point) {
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
        if (self.point.x - obj_point.x > self.radius) & ((actions.get(&self.controller.Up) == Some(&true)) | (actions.get(&self.controller.Down) == Some(&true))){
            self.point.x += dt * self.speed;
        }
        if (obj_point.x - self.point.x > self.radius) & ((actions.get(&self.controller.Up) == Some(&true)) | (actions.get(&self.controller.Down) == Some(&true))){
            self.point.x -= dt * self.speed;
        }
        if (self.point.y - obj_point.y > self.radius) & ((actions.get(&self.controller.Left) == Some(&true)) | (actions.get(&self.controller.Right) == Some(&true))){
            self.point.y += dt * self.speed;
        }
        if (obj_point.y - self.point.y > self.radius) & ((actions.get(&self.controller.Left) == Some(&true))| (actions.get(&self.controller.Right) == Some(&true))){
            self.point.y -= dt * self.speed;
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
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    //     // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
