use crate::models::{World, Player, Bomb, Wall, Block, Fire, Item};
use std::collections::HashMap;

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
            for w in world.blocks.iter(){//all_objs.iter
                match p.collide_with_block(w){
                    0 => (),
                    // 11 => self.player_to_player(p,b),
                    // 12 => player_to_bomb(p,b,dt,actions),
                    14 => player_to_block(p,w,dt,actions),
                    _ => (),
                }
            }
            for f in world.fire.iter(){
                match p.collide_with_fire(f){
                    15 => player_to_fire(p),
                    _ => (),
                }
            }
            for i in world.item.iter_mut(){
                match p.collide_with_item(i){
                    16 => player_to_item(p,i),
                    _ => (),
                }
            }

        }
        for f in world.fire.iter_mut(){
            for o in world.bomb.iter_mut(){
                if (f.point.x == o.point.x) && (f.point.y == o.point.y) {
                    o.ttl = 0.0;
                }
            }
            for o in world.walls.iter(){
                if (f.point.x == o.point.x) && (f.point.y == o.point.y) {
                    f.dir.up = 0;
                    f.dir.down= 0;
                    f.dir.right= 0;
                    f.dir.left= 0;
                    fire_to_wall(f);
                }
            }
            for o in world.blocks.iter_mut(){
                if (f.point.x == o.point.x) && (f.point.y == o.point.y) {
                    f.dir.up = 0;
                    f.dir.down= 0;
                    f.dir.right= 0;
                    f.dir.left= 0;
                    fire_to_block(o);
                }
            }
            for o in world.item.iter_mut(){
                if (f.point.x == o.point.x) && (f.point.y == o.point.y) {
                    fire_to_item(o);
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
pub fn player_to_block(player: &mut Player, block: &Block, dt: f64, actions: &HashMap<String, bool>){
    player.relocate(dt,actions,&block.point);
}
pub fn player_to_fire(player: &mut Player){
    player.die();
}
pub fn player_to_item(player: &mut Player, item: &mut Item){
    player.get_item(item);
    item.remove();
}
pub fn fire_to_block(block: &mut Block) {
    block.broken();
}
pub fn fire_to_wall(fire: &mut Fire) {
    fire.roll_back();
}
pub fn fire_to_item(item: &mut Item){
    // item.remove();
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

