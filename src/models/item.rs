use crate::geometry::{Point, Position};
use crate::controller::Event;
use std::collections::VecDeque;

pub struct Item {
    pub id: i32,
    pub radius: i32,
    pub alive: bool,
    pub point: Point,
    pub ability: i32,//enum
}

impl Item {
    pub fn new(id: i32, point: Point) -> Self {
        Item{
            id: id,
            radius: 24,
            point: point,
            alive: true,
            ability: 0,//enum
        }
    }
    pub fn update(&mut self, event: &mut VecDeque<Event>) {
        let id = self.id;
        if !self.alive {
            event.push_back(Event::Disappearance{id});
        }
    }
    pub fn remove(&mut self){
        self.alive = false;
    }
    pub fn draw(&self){
        draw_item_boots(self.point.x, self.point.y);
    }
}


use wasm_bindgen::prelude::*;
#[wasm_bindgen(module = "/src/javascript/canvas.js")]
extern "C" {
    pub fn draw_item_boots(x: i32, y: i32);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
