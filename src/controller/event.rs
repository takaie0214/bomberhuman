use crate::geometry::Dir;
use crate::geometry::Point;

pub enum Event{
    SetBomb{id: i32, x: i32, y: i32, firepower: i32},
    GenItem{id: i32, point: Point},
    Explosion{fid: i32, bid: i32, x: i32, y: i32, dir: Dir},
    FireSpread{fid: i32, bid: i32, x: i32, y: i32, dir: Dir},
    Disappearance{id: i32},
}

