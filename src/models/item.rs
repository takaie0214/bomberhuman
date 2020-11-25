extern crate rand;
use crate::geometry::Point;
use crate::controller::Event;
use std::collections::VecDeque;
use rand::Rng;

pub struct Item {
    pub id: i32,
    pub radius: i32,
    pub alive: bool,
    pub point: Point,
    pub ability: i32,//enum
    pub elapsed_time: f64,
}

impl Item {
    pub fn new(id: i32, point: Point) -> Self {
        let mut rng = rand::thread_rng();
        let rn: i32 = rng.gen();
        Item{
            id: id,
            radius: 24,
            point: point,
            alive: true,
            ability: (rn%3).abs(),
            elapsed_time: 0.0,
        }
    }
    pub fn update(&mut self, dt: f64, event: &mut VecDeque<Event>) {
        let id = self.id;
        if !self.alive {
            event.push_back(Event::Disappearance{id});
        }
        self.elapsed_time += dt;
    }
    pub fn remove(&mut self){
        self.alive = false;
    }
    pub fn draw(&self){
        match self.ability {
            0 => draw_item_boots(self.point.x, self.point.y),
            1 => draw_item_fire(self.point.x, self.point.y),
            2 => draw_item_bomb(self.point.x, self.point.y),
            _ => (),
        }
    }
}


use wasm_bindgen::prelude::*;
#[wasm_bindgen(module = "/src/javascript/canvas.js")]
extern "C" {
    pub fn draw_item_boots(x: i32, y: i32);
}
#[wasm_bindgen(module = "/src/javascript/canvas.js")]
extern "C" {
    pub fn draw_item_fire(x: i32, y: i32);
}
#[wasm_bindgen(module = "/src/javascript/canvas.js")]
extern "C" {
    pub fn draw_item_bomb(x: i32, y: i32);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
