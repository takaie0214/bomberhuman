/// Active actions (toggled by user input)
#[derive(Default, Clone)]
pub struct Controller{
    pub up: bool,
    pub down: bool,
    pub right: bool,
    pub left: bool,
    pub button1: bool,
    pub button2: bool,
}
impl Controller {
    pub fn new() -> Self {
        Controller {
            up: false,
            down: false,
            right: false,
            left: false,
            button1: false,
            button2: false,
        }
    }
}



