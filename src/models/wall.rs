use crate::geometry::{Point, Size};

pub struct Wall {
    point: Point,
}

impl Wall {
    pub fn new(point: Point) -> Self {
        Wall {
            point: point,
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

