use crate::geometry::Point;

pub struct Block {
    pub point: Point,
    pub id: i32,
    pub radius: i32,
    pub alive: bool,
}

impl Block{
    pub fn new(point: Point) -> Self {
        Block {
            point: point,
            id: 400,
            radius: 24,
            alive: true,
        }
    }

    pub fn update(&self) {
        //new Item
    }

    pub fn draw(&self){
        draw_block(self.point.x, self.point.y);
    }
}

use wasm_bindgen::prelude::*;
#[wasm_bindgen(module = "/src/javascript/canvas.js")]
extern "C" {
    pub fn draw_block(x: i32, y: i32);
}

