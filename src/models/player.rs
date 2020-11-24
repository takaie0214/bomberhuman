use crate::geometry::{Point, Position};
use crate::controller::{Controller, Event};
use crate::models::{Bomb, Wall, Block, Fire, Item};
use std::collections::VecDeque;

/// The `Player` is the rocket controlled by the user
//#[derive(Default)]
pub struct Player {
    pub id: i32,
    alive: bool,
    pub bomb_count: i32,
    pub point: Point,
    old_point: Point,
    dir : String,
    idle: bool,
    walk_count: i32,
    prev_dir: String,
    speed: f64,
    bomb_ct: f64,
    pub firepower: i32,
    pub bomb_quantity: i32,
    pub radius: i32,
}

impl Player {
    /// Create a new `Player` with a random position and direction
    pub fn new(id: i32, point: Point) -> Self {
        let speed: f64 = 160.0;
        let firepower: i32 = 1;
        let bomb_quantity: i32 = 1;
        let radius: i32 = 24;

        Player {
            id: id,
            alive: true,
            bomb_count: 0,
            point: point,
            old_point: point,
            dir : String::from("down"),
            prev_dir: String::from("down"),
            idle: true,
            walk_count: 0,
            speed: speed,
            bomb_ct: 0.0,
            firepower: firepower,
            bomb_quantity: bomb_quantity,
            radius: radius,
        }
    }
    pub fn update(&mut self, dt: f64, controller: &Controller , event: &mut VecDeque<Event>){
        if !self.alive {
            return ;
        }
        let mut x = 0;
        let mut y = 0;
        self.old_point.x = self.point.x;
        self.old_point.y = self.point.y;

        if controller.up  {
            y -= (dt * self.speed) as i32;
        }

        else if controller.down {
            y += (dt * self.speed) as i32;
        }

        else if controller.right {
            x += (dt * self.speed) as i32;
        }

        else if controller.left {
            x -= (dt * self.speed) as i32;
        }

        self.point.y += y;
        self.point.x += x;

        if y < 0{
            self.idle=false;
            self.dir = "up".to_string();
        }else if y > 0 {
            self.idle=false;
            self.dir = "down".to_string();
        }else if x > 0 {
            self.idle=false;
            self.dir = "right".to_string();
        }else if x < 0 {
            self.idle=false;
            self.dir = "left".to_string();
        }else{
            self.idle=true;
        }

        if &self.prev_dir == &self.dir {
        }else {
            self.prev_dir =  String::from(&self.dir.clone());
        }

        self.bomb_ct -= dt;
        if controller.button1 && self.bomb_quantity > self.bomb_count && self.bomb_ct < 0.0{

         //   let new_bomb = Bomb::new(200+self.id%10*10+self.bomb_count,((self.point.x) as i32 / 50*50 + 25)as f64,((self.point.y) as i32 /50*50 + 25) as f64);
            let x = self.point.x  / 50*50 + 25;
            let y = self.point.y  /50*50 + 25;
            let id = 200+self.id%10*10+self.bomb_count;
            let firepower = self.firepower;

            event.push_back(Event::SetBomb{id,x,y,firepower});
            self.bomb_ct = 1.0;
            self.bomb_count += 1;
            self.bomb_count %= 5;
        }

        // if !self.alive {
        //     let id = self.id;
        //     event.push_back(Event::Disappearance{id});
        // }
    }
    pub fn draw(&mut self){
        if !self.alive {
            return;
        }
        let mut y = 0;
        let mut x = 0;

        if self.idle == true {self.walk_count = 0;}
        else {self.walk_count += self.speed as i32;}
        //log(&self.walk_count.to_string());

        // draw_player(self.point.x, self.point.y);
        if self.dir == "up" {y=3}
        if self.dir == "down" {y=0}
        if self.dir == "left" {y=1}
        if self.dir == "right" {y=2}

        if self.walk_count > 0  {x=2}
        if self.walk_count > 800  {x=1}
        if self.walk_count > 1600  {x=0}
        if self.walk_count > 2400  {x=1; self.walk_count=-9;}

        let id = "player".to_string() + &(self.id-100).to_string();
        draw_player_animation(&id, x, y, self.point.x, self.point.y);
//        self.sprite.animate(self.point.x, self.point.y, &self.dir, self.idle, self.walking);
    }

    pub fn collide_with_bomb(&mut self,obj: &mut Bomb) -> i32 {
        let radii = self.radius + obj.radius;
        // if (((self.point.x - obj.point.x).abs() < 47.0) & ((self.point.y - obj.point.y).abs() < 47.0)){
        //     return 0;
        // }
        if ((self.point.x - obj.point.x).abs() < radii) & ((self.point.y - obj.point.y).abs() < radii){
            if obj.on_player[(self.id%10 - 1) as usize] {
                return 0;
            }
                // let test = self.id;
                // let testa = obj.id;
                // log(&test.to_string());
                // log(&testa.to_string());
            else {
                return self.id / 100 * 10 + obj.id / 100;

            }
        }
        else {
            obj.on_player[(self.id%10 - 1) as usize] = false;
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
        if ((self.point.x - obj.point.x).abs() < radii) & ((self.point.y - obj.point.y).abs() < radii){
            return self.id / 100 * 10 + obj.id / 100;
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
        if ((self.point.x - obj.point.x).abs() < radii) & ((self.point.y - obj.point.y).abs() < radii){
            return self.id / 100 * 10 + obj.id / 100;
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
    pub fn collide_with_item(&self,obj: &Item) -> i32 {
        let radii = self.radius + obj.radius;
        if ((self.point.x - obj.point.x).abs() < radii) & ((self.point.y - obj.point.y).abs() < radii){
            return self.id / 100 * 10 + obj.id / 100;
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
        if ((self.point.x - obj.point.x).abs() < radii) & ((self.point.y - obj.point.y).abs() < radii){
            return self.id / 100 * 10 + obj.id / 100;
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

    pub fn get_item(&mut self, item: &Item) {
        match item.ability {
        0 => {self.speed += 40.0;},
        1 => {self.firepower += 1},
        2 => {self.bomb_quantity += 1},
        _ => (),
        }
    }


    pub fn relocate(&mut self, dt: f64, controller: &Controller, obj_point: &Point) {
        self.point.x = self.old_point.x;
        self.point.y = self.old_point.y;

        if (self.point.x - obj_point.x > self.radius) && (controller.up || controller.down) {
            // self.point.x += (dt * self.speed) as i32;
            self.point.x = self.point.x / 50 * 50 + 25;
        }
        else if (obj_point.x - self.point.x > self.radius) && (controller.up || controller.down) {
            // self.point.x -= (dt * self.speed) as i32;
            self.point.x = self.point.x / 50 * 50 + 25;
        }
        else if (self.point.y - obj_point.y > self.radius) && (controller.left || controller.right) {
            // self.point.y += (dt * self.speed) as i32;
            self.point.y = self.point.y /50 * 50 + 25;
        }
        else if (obj_point.y - self.point.y > self.radius) && (controller.left || controller.right) {
            // self.point.y -= (dt * self.speed) as i32;
            self.point.y = self.point.y /50 * 50 + 25;
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
