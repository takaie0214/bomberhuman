use crate::models::Bomb;
use crate::geometry::{Point, Size};

pub enum EventType {
    SetBomb{id: i32, x: i32, y: i32},
    Explosion,
}


pub struct Event {
}

