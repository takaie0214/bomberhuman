use crate::geometry::{Point, Position, Dir};

use crate::controller::Event;
use std::collections::VecDeque;

pub struct Bomb {
    pub id: i32,
    pub radius: i32,
    pub ttl: f64,
    pub point: Point,
    pub firepower: i32,
    pub on_player: Vec<bool>,
}

impl Bomb {
    pub fn new(id: i32, x: i32, y: i32, firepower: i32, on_player: Vec<bool>) -> Self {
        Bomb{
            id: id,
            radius: 24,
            point: Point::new(x,y),
            ttl: 5.0,
            firepower: firepower,
            on_player: on_player,
        }
    }
    pub fn update(&mut self, dt: f64, event: &mut  VecDeque<Event>) {
        self.ttl -= dt;
        if self.ttl < 0.0 {
            let bid = self.id;
            let fid = self.id - 200 + 500;
            let x =  self.x();
            let y =  self.y();
            let firepower = self.firepower;
            let dir = Dir::new(firepower, firepower, firepower, firepower);
            event.push_back(Event::Explosion{fid, bid, x, y, dir});
        }
    }
    // pub fn detonate(&mut self, fire: &mut Fire){
    //     let fid = self.id - 200 + 500;
    //     let bid = self.id;
    //     let x = self.x();
    //     let y = self.y();
    //     event.push_back(Event::Explosion{fid, bif, x, y, dir});
    // }
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

