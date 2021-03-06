use crate::geometry::{Point, Position, Dir};
use crate::models::Fire;
use crate::controller::Event;
use std::collections::VecDeque;

pub struct Bomb {
    pub id: i32,
    pub radius: i32,
    pub ttl: f64,
    pub point: Point,
    pub firepower: i32,
    pub on_player: Vec<bool>,
    pub dir: Dir,
}

impl Bomb {
    pub fn new(id: i32, x: i32, y: i32, firepower: i32, on_player: Vec<bool>) -> Self {
        Bomb{
            id: id,
            radius: 24,
            point: Point::new(x,y),
            firepower: firepower,
            ttl: 5.0,
            on_player: on_player,
            dir: Dir::new(firepower, firepower, firepower, firepower),
        }
    }
    pub fn update(&mut self, dt: f64, event: &mut  VecDeque<Event>) {
        self.ttl -= dt;
        if self.ttl < 0.0 {
            let bid = self.id;
            let fid = self.id - 200 + 500;
            let x =  self.x();
            let y =  self.y();
            let dir = self.dir;
            event.push_back(Event::Explosion{fid, bid, x, y, dir});
        }
    }
    pub fn detonate(&mut self, fire: &mut Fire){
        let down = if fire.dir.up == 0 {self.firepower} else {0};
        let up = if fire.dir.down == 0 {self.firepower} else {0};
        let left = if fire.dir.right == 0 {self.firepower} else {0};
        let right = if fire.dir.left == 0 {self.firepower} else {0};
        self.dir = Dir::new(up,down,right,left);
        self.ttl = 0.0;
    }
    pub fn draw(&self){
        let mut x = 0;
        let y = 0;

        if self.ttl < 1.0{
            x = 2;
        }else if self.ttl < 3.0 {
            x = 1;
        }

        draw_bomb(x, y,self.point.x, self.point.y);
    }
}
impl Position for Bomb{
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
    pub fn draw_bomb(recX:i32, recY:i32,x: i32, y: i32);
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

