use crate::geometry::{Point, Size};
use crate::controller::Actions;

/// The `Player` is the rocket controlled by the user
#[derive(Default)]
pub struct Player {
    point: Point,
    speed: f64
}

impl Player {
    /// Create a new `Player` with a random position and direction
    pub fn new(point: Point) -> Self {
        let speed: f64 = 20.0;
        Player {point: point, speed: speed}
    }
    pub fn update(&mut self, dt: f64, actions: &Actions){
        let a: Point = Point::new(2.0, 2.0);
        self.point.translate(&a);


        if actions.move_up {
            *self.y_mut() -= dt * self.speed;
        }

        if actions.move_down {
            *self.y_mut() += dt * self.speed;
        }

        if actions.move_right {
            *self.x_mut() += dt * self.speed;
        }

        if actions.move_left {
            *self.x_mut() -= dt * self.speed;
        }
    }
    pub fn x(&self) -> f64{
        self.point.x
    }
    pub fn y(&self) -> f64{
        self.point.y
    }
    pub fn x_mut(&mut self) -> &mut f64{
        &mut self.point.x
    }
    pub fn y_mut(&mut self) -> &mut f64{
        &mut self.point.y
    }
}

