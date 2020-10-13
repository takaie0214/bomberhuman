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
    pub fn new(point: Point, speed: f64) -> Self {
        Player {point: point, speed: speed}
    }
    pub fn update(&mut self, dt: f64, actions: &Actions){

        if actions.ArrowUp {
           // *self.y_mut() -= dt * self.speed;
            *self.y_mut() -= dt * self.speed;;
        }

        if actions.ArrowDown{
            *self.y_mut() += dt * self.speed;
        }

        if actions.ArrowRight {
            *self.x_mut() += dt * self.speed;
        }

        if actions.ArrowLeft {
            *self.x_mut() -= dt * self.speed;
        }
    }
    pub fn x(&self) -> f64 {
        self.point.x
    }
    pub fn y(&self) -> f64 {
        self.point.y
    }
    pub fn x_mut(&mut self) -> &mut f64 {
        &mut self.point.x
    }
    pub fn y_mut(&mut self) -> &mut f64 {
        &mut self.point.y
    }
}

