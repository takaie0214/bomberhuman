use crate::models::World;
use crate::models::Player;
use crate::models::Bomb;
use crate::models::Wall;
use crate::controller::Actions;
use std::collections::HashMap;
use crate::geometry::Point;

use wasm_bindgen::prelude::*;

pub struct Collision {
    // players: Vec<Player>,
    // bomb: Vec<Bomb>,
    // walls: Vec<Wall>

}

impl Collision {
    // pub fn new(world: World) -> Self {
    //     Collision{
    //         players = world.players,
    //         bomb = world.bomb,
    //         walls = world.walls
    //     }
    // }

    pub fn collision_with (dt: f64, actions: &HashMap<String, bool>, world: &mut World) {
        for p in world.players.iter_mut(){
            for b in world.bomb.iter(){//all_objs.iter
                match p.collide_with_bomb(b){
                    0 => (),
                    // 11 => self.player_to_player(p,b),
                    12 => player_to_bomb(p,b,dt,actions),
                    // 13 => self.player_to_wall(p,b,dt,actions)
                    _ => (),
                }
            }
            for w in world.walls.iter(){//all_objs.iter
                match p.collide_with_wall(w){
                    0 => (),
                    // 11 => self.player_to_player(p,b),
                    // 12 => player_to_bomb(p,b,dt,actions),
                    13 => player_to_wall(p,w,dt,actions),
                    _ => (),
                }
            }
        }
    }

    // pub fn player_to_player(player1: Player, player2: Player){
    // }
}
pub fn player_to_bomb(player: &mut Player, bomb: &Bomb, dt: f64, actions: &HashMap<String, bool>){
    player.relocate(dt,actions,&bomb.point);
}
pub fn player_to_wall(player: &mut Player, wall: &Wall, dt: f64, actions: &HashMap<String, bool>){
    player.relocate(dt,actions,&wall.point);
}
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

