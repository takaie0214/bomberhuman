use crate::geometry::Point;

pub struct Wall {
    pub point: Point,
    pub id: i32,
    pub radius: i32,
}

impl Wall {
    pub fn new(point: Point) -> Self {
        Wall {
            point: point,
            id: 300,
            radius: 24,
        }
    }
    pub fn draw(&self){
        draw_wall(self.point.x, self.point.y);
    }
}

use wasm_bindgen::prelude::*;
#[wasm_bindgen(module = "/src/javascript/canvas.js")]
extern "C" {
    pub fn draw_wall(x: i32, y: i32);
}

