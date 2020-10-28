use crate::geometry::Point;

use crate::controller::{Actions, Controller, Event, EventType};

pub struct Bomb {
    ttl: f64,
    point: Point,
}

impl Bomb {
    pub fn new(point: Point) -> Self {
        Bomb{
            point: point,
            ttl: 5.0,
        }
    }
    pub fn update(&mut self, dt: f64, event: &mut  Vec<EventType>) {
        self.ttl -= dt;
//        if (self.ttl < 0.0) {
//            event.push(EventType::Explosion);
//        }
    }
    pub fn draw(&self){
        let mut x = 0;
        let mut y = 0;

        if (self.ttl < 1.0){
            x = 2;
        }else if (self.ttl < 3.0) {
            x = 1;
        }

        draw_bomb(x, y,self.point.x, self.point.y);
    }
}

use wasm_bindgen::prelude::*;
#[wasm_bindgen(module = "/src/javascript/canvas.js")]
extern "C" {
    pub fn draw_bomb(recX:i32, recY:i32,x: f64, y: f64);
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}
