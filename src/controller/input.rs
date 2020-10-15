/// Active actions (toggled by user input)
#[derive(Default)]
pub struct Actions {
    pub ArrowUp: bool,
    pub ArrowDown: bool,
    pub ArrowRight: bool,
    pub ArrowLeft: bool,
}

pub struct Controller{
    pub Up: String,
    pub Down: String,
    pub Right: String,
    pub Left: String,
}
impl Controller {
    pub fn new(up: &str, down: &str, right: &str, left: &str) -> Self {
        Controller {
            Up: up.to_string(),
            Down: down.to_string(),
            Right: right.to_string(),
            Left: left.to_string(),
        }
    }
}



