use crate::models::Bomb;
use crate::geometry::{Point, Size};

pub enum EventType {
    SetBomb(Bomb),
    Explosion,
}


pub struct Event {
}

