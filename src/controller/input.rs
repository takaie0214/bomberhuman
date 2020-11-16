/// Active actions (toggled by user input)
#[derive(Default)]
pub struct Controller{
    pub Up: String,
    pub Down: String,
    pub Right: String,
    pub Left: String,
    pub A: String,
}
impl Controller {
    pub fn new(up: &str, down: &str, right: &str, left: &str, a: &str) -> Self {
        Controller {
            Up: up.to_string(),
            Down: down.to_string(),
            Right: right.to_string(),
            Left: left.to_string(),
            A: a.to_string(),
        }
    }
}



