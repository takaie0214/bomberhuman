/// Active actions (toggled by user input)
#[derive(Default)]
pub struct Controller{
    pub up: String,
    pub down: String,
    pub right: String,
    pub left: String,
    pub button1: String,
}
impl Controller {
    pub fn new(up: &str, down: &str, right: &str, left: &str, button1: &str) -> Self {
        Controller {
            up: up.to_string(),
            down: down.to_string(),
            right: right.to_string(),
            left: left.to_string(),
            button1: button1.to_string(),
        }
    }
}



