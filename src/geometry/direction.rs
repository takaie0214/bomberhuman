pub struct Dir {
    pub up: i32,
    pub down: i32,
    pub right: i32,
    pub left: i32,
}

impl Dir {
    pub fn new(up: i32, down:i32, right:i32, left:i32) -> Self {
        Dir {
            up: up,
            down: down,
            right: right,
            left: left,
        }
    }
}
