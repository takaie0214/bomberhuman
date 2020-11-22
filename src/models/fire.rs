use crate::geometry::{Point, Position, Dir};

use std::collections::VecDeque;
use crate::controller::Event;

pub struct Fire {
    pub id: i32,
    pub bid: i32,
    pub radius: i32,
    pub point: Point,
    pub ttl: f64,
    pub dir: Dir,
    pub on_wall: bool,
}

impl Fire {
    pub fn new(id: i32, bid: i32, x: i32, y: i32, dir: Dir) -> Self {
        Fire {
            radius: 24,
            id: id,
            bid: bid,
            point: Point::new(x,y),
            ttl: 1.6,
            dir: dir,
            on_wall: false,
        }
    }
    pub fn update(&mut self, dt: f64, event: &mut  VecDeque<Event>) {
        let id = self.id;
        let bid = self.bid;
        if self.dir.up > 0 {
            let x = self.x();
            let y = self.y() - 50;
            let dir = self.dir.up - 1;
            let new_dir = Dir::new(dir, 0, 0, 0);
            event.push_back(Event::FireSpread{fid: id, bid: bid, x: x, y: y, dir:new_dir});
            self.dir.up = 0;
        }
        if self.dir.down > 0 {
            let  x = self.x();
            let  y = self.y() + 50;
            let dir = self.dir.down - 1;
            let new_dir = Dir::new(0, dir, 0, 0);
            event.push_back(Event::FireSpread{fid: id, bid: bid, x: x, y: y, dir:new_dir});
            self.dir.down= 0;
        }
        if self.dir.right > 0 {
            let x = self.x() + 50;
            let y = self.y();
            let dir = self.dir.right - 1;
            let new_dir = Dir::new( 0, 0, dir, 0);
            event.push_back(Event::FireSpread{fid: id, bid: bid, x: x, y: y, dir:new_dir});
            self.dir.right = 0;
        }
        if self.dir.left > 0 {
            let x = self.x() - 50;
            let y = self.y();
            let dir = self.dir.left- 1;
            let new_dir = Dir::new(0, 0, 0, dir);
            event.push_back(Event::FireSpread{fid: id, bid: bid, x: x, y: y, dir:new_dir});
            self.dir.left = 0;
        }

        self.ttl -= dt;
        if self.ttl < 0.0 {
            event.push_back(Event::Disappearance{id});
        }
    }
    pub fn roll_back(&mut self){
        self.on_wall = true;
    }
    pub fn draw(&self){
        let mut x = 0;
        let y = 0;

        if self.ttl < 0.4 {
            x = 3;
        }else if self.ttl < 0.8 {
            x = 2;
        }else if self.ttl < 1.2 {
            x = 1;
        }

        draw_fire(x, y, self.point.x, self.point.y);
    }
}
impl Position for Fire{
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
    pub fn draw_fire(recX:i32, recY:i32,x: i32, y: i32);
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

