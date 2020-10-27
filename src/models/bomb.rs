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
            ttl: 100.0,
        }
    }
    pub fn update(&mut self, dt: f64, event: &mut  Vec<EventType>) {
        self.ttl -= dt;
//        if (self.ttl < 0.0) {
//            event.push(EventType::Explosion);
//        }
    }
    pub fn draw(&self){
        draw_bomb(self.point.x, self.point.y);
    }
}

use wasm_bindgen::prelude::*;
#[wasm_bindgen(module = "/src/javascript/canvas.js")]
extern "C" {
    pub fn draw_bomb(x: f64, y: f64);
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}
