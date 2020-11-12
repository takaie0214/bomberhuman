use crate::geometry::{Point, Size, Position, Dir};

use crate::controller::{Actions, Controller, Event, EventType};

pub struct Bomb {
    pub id: i32,
    pub radius: i32,
    pub ttl: f64,
    pub point: Point,
}

impl Bomb {
    pub fn new(id: i32, x: i32, y: i32) -> Self {
        Bomb{
            id: id,
            radius: 24,
            point: Point::new(x,y),
            ttl: 5.0,
        }
    }
    pub fn update(&mut self, dt: f64, event: &mut  Vec<EventType>) {
        self.ttl -= dt;
        if (self.ttl < 0.0) {
            let id = self.id - 200 + 500;
            let x =  self.x();
            let y =  self.y();
            let dir = Dir::new(3, 3, 3, 3);
            event.push(EventType::Explosion{id, x, y, dir});
        }
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
impl Position for Bomb{
    fn x(&self) -> i32{
        self.point.x

    }
    fn y(&self) -> i32{
        self.point.y

    }
}

use wasm_bindgen::prelude::*;
#[wasm_bindgen(module = "/src/javascript/canvas.js")]
extern "C" {
    pub fn draw_bomb(recX:i32, recY:i32,x: i32, y: i32);
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

