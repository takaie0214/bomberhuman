use crate::models::Bomb;
use crate::geometry::{Point, Size, Dir};

pub enum EventType {
    SetBomb{id: i32, x: i32, y: i32},
    Explosion{id: i32, x: i32, y: i32, dir: Dir},
    Disappearance{id: i32},
}


pub struct Event {
}

