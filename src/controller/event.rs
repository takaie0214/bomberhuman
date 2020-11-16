use crate::geometry::Dir;

pub enum Event{
    SetBomb{id: i32, x: i32, y: i32},
    Explosion{id: i32, x: i32, y: i32, dir: Dir},
    Disappearance{id: i32},
}

