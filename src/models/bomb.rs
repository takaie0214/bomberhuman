use crate::geometry::Point;

use crate::controller::{Actions, Controller, Event, EventType};

pub struct Bomb {
    pub id: i32,
    pub radius: f64,
    pub ttl: f64,
    pub point: Point,
}

impl Bomb {
    pub fn new(point: Point) -> Self {
        Bomb{
            id: 200,
            radius: 20.0,
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
