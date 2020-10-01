use pcg_rand::Pcg32Basic;
use rand::SeedableRng;

use geometry::{Size};//Position
use models::World;

/// The data structure that contains the state of the game
pub struct GameState {
    /// The world contains everything that needs to be drawn
    pub world: World,
    // The current score of the player//this is doc comment
    //pub score: u32
}

impl GameState {
    /// Returns a new `GameState` containing a `World` of the given `Size`
    pub fn new(size: Size) -> GameState {
        let mut rng = Pcg32Basic::from_seed([42, 42]);
        GameState {
            world: World::new(&mut rng, size),
            //score: 0
        }
    }

    // Reset our game-state
    //コリジョンがないからリセット不要
    //pub fn reset(&mut self) {
    //    let mut rng = Pcg32Basic::from_seed([42, 42]);

    //    // Reset player position
    //    *self.world.player.x_mut() = self.world.size.random_x(&mut rng);
    //    *self.world.player.y_mut() = self.world.size.random_y(&mut rng);

        // Reset score
        //self.score = 0;

        // Remove all enemies and bullets
 //       self.world.bullets.clear();
 //       self.world.enemies.clear();
 //   }
}
