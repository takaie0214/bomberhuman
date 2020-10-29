use crate::models::Bomb;
use crate::geometry::{Point, Size};

pub enum EventType {
    SetBomb{id: i32, x: f64, y: f64},
    Explosion,
}


pub struct Event {
}

