use crate::geometry::Point;
use crate::controller::Event;
use std::collections::VecDeque;
use rand::Rng;

pub struct Block {
    pub point: Point,
    pub id: i32,
    pub radius: i32,
    pub alive: bool,
}

impl Block{
    pub fn new(id: i32,point: Point) -> Self {
        Block {
            point: point,
            id: id,
            radius: 24,
            alive: true,
        }
    }

    pub fn update(&self,event: &mut VecDeque<Event>) {
        //new Item
        let id = self.id;
        let point = self.point;
        let mut rng = rand::thread_rng();
        let rn: i32 = rng.gen();
        if !self.alive {
            event.push_back(Event::Disappearance{id});
            let id = self.id + 200;
            if rn%2 == 0 {
                event.push_back(Event::GenItem{id,point})
            }
        }
        // let tmp: &str = &id.to_string();
        // log(tmp);
    }

    pub fn broken(&mut self) {
        self.alive = false;
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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
