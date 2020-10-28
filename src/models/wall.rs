use crate::geometry::{Point, Size};

pub struct Wall {
    pub point: Point,
    pub id: i32,
    pub radius: f64,
}

impl Wall {
    pub fn new(point: Point) -> Self {
        Wall {
            point: point,
            id: 300,
            radius: 20.0,
        }
    }
    pub fn draw(&self){
        draw_wall(self.point.x, self.point.y);
    }
}

use wasm_bindgen::prelude::*;
#[wasm_bindgen(module = "/src/javascript/canvas.js")]
extern "C" {
    pub fn draw_wall(x: f64, y: f64);
}

