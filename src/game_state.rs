use crate::geometry::Size;
use crate::models::World;
use crate::controller::Actions;


// The data structure that contains the state of the game
pub struct GameState {
    // The world contains everything that needs to be drawn
    pub world: World,
}

impl GameState {
    /// Returns a new `GameState` containing a `World` of the given `Size`
    pub fn new(size: Size) -> GameState {
        GameState {
            world: World::new(size),
        }
    }
    pub fn update(&mut self, dt: f64,actions: &Actions){
        self.world.update(dt, actions);
    }
}
