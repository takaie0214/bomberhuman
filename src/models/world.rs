use crate::geometry::{Point, Size};
use rand::Rng;
use pcg_rand::Pcg32Basic;
use rand::SeedableRng;
use crate::models::{Player, Bomb, Wall,  Block, Fire, Item};
use std::collections::HashMap;
use std::collections::VecDeque;
use crate::controller::{Controller, Event};

/// A model that contains the other models and renders them
pub struct World {
    pub players: Vec<Player>,
    pub bomb: Vec<Bomb>,
    pub walls: Vec<Wall>,
    pub blocks: Vec<Block>,
    pub fire: Vec<Fire>,
    pub item: Vec<Item>,
    pub size: Size,
    pub event: VecDeque<Event>,
}

impl World {
    pub fn new(size: Size) -> World {
        let mut players = Vec::new();

        let id1 = 101;
        let point1 = Point::new(75, 75);
        players.push(Player::new(id1, point1));

        let id2 = 102;
        let point2 = Point::new(675, 575);
        players.push(Player::new(id2, point2));

        let id3 = 103;
        let point3 = Point::new(675, 75);
        players.push(Player::new(id3, point3));

        let id4 = 104;
        let point4 = Point::new(75, 575);
        players.push(Player::new(id4, point4));

        let wall_bmp = [[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,1,0,1,0,1,0,1,0,1,0,1,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,1,0,1,0,1,0,1,0,1,0,1,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,1,0,1,0,1,0,1,0,1,0,1,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,1,0,1,0,1,0,1,0,1,0,1,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,1,0,1,0,1,0,1,0,1,0,1,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]];

        let block_bmp = [[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,1,1,1,1,1,1,1,1,1,0,0,0],
        [0,0,0,1,0,1,0,1,0,1,0,1,0,0,0],
        [0,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
        [0,1,0,1,0,1,0,1,0,1,0,1,0,1,0],
        [0,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
        [0,1,0,1,0,1,0,1,0,1,0,1,0,1,0],
        [0,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
        [0,1,0,1,0,1,0,1,0,1,0,1,0,1,0],
        [0,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
        [0,0,0,1,0,1,0,1,0,1,0,1,0,0,0],
        [0,0,0,1,1,1,1,1,1,1,1,1,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]];

        let mut walls = Vec::new();
        for (index1, val1) in wall_bmp.iter().enumerate() {
            for (index2, val2) in val1.iter().enumerate() {
                if *val2 == 1 {
                    walls.push(Wall::new(Point::new(50 * index2 as i32 + 25, 50 * index1 as i32 + 25)));
                }
            }
        }

        let mut blocks = Vec::new();
        let mut rng = Pcg32Basic::from_seed([42, 42]);
        let mut block_id = 400;
        for (index1, val1) in block_bmp.iter().enumerate(){
            for (index2,val2) in val1.iter().enumerate(){
                if *val2 == 1 && rng.gen_range(0, 9) > 0 {
                    blocks.push(Block::new(block_id,Point::new(50 * index2 as i32 + 25, 50 * index1 as i32 + 25)));
                    block_id += 1;
                }
            }
        }

        let item = Vec::new();
        let fire = Vec::new();

        World {
            players: players,
            bomb: vec![],
            walls: walls,
            blocks: blocks,
            fire: fire,
            item: item,
            size: size,
            event: VecDeque::new(),
        }
    }

    pub fn update(&mut self, dt: f64, controllers: &Vec<Controller>) {
        for p in &mut self.players {
            p.update(dt, &controllers[(p.id - 101) as usize], &mut self.event);
        }
        for b in &mut self.bomb {
            b.update(dt,  &mut self.event);
        }
        for b in &mut self.blocks {
            b.update(&mut self.event);
        }
        for f in &mut self.fire {
            f.update(dt,  &mut self.event);
        }
        for i in &mut self.item {
            i.update(&mut self.event);
        }
        while self.event.len() != 0 {
            match self.event.pop_front() {
                None => (),
                Some(event) =>
                    match event {
                        Event::SetBomb{id,x,y,firepower} => {
                            for b in &self.bomb{
                                if b.point.x == x && b.point.y == y {
                                    return ();
                                }
                            }
                            self.bomb.push(Bomb::new(id,x,y,firepower));
                        },
                        Event::GenItem{id,point} => {
                            self.item.push(Item::new(id,point))
                        }
                        Event::Explosion{fid, bid, x, y, dir} => {
                            self.bomb.retain(|elem| elem.id != bid);//idはfireid,bombidが必要
                            self.fire.push(Fire::new(fid, bid, x, y, dir));
                            self.players[(bid%100/10 - 1) as usize].bomb_count -= 1;
                            // let tmp: &str = &id.to_string();
                            // let tmp2: &str = &self.bomb[0].id.to_string();
                            // log(tmp);
                            // log(tmp2);
                            // let size: &str = &self.fire.len().to_string();
                            // log(size);
                        }
                        Event::FireSpread{fid, bid, x, y, dir} => {
                            self.fire.push(Fire::new(fid, bid, x, y, dir));
                        }
                        Event::Disappearance{id} => {
                            // self.fire.iter().map(|elem| {let tmp: &str = &elem.id.to_string();log(tmp);});
                            self.fire.retain(|elem| elem.id != id);
                            self.blocks.retain(|elem| elem.id != id);
                            self.players.retain(|elem| elem.id != id);
                            self.item.retain(|elem| elem.id != id);
                            // let size: &str = &self.fire.len().to_string();
                            // log(size);
                            // let tmp: &str = &id.to_string();
                            // let tmp2: &str = &self.fire[0].id.to_string();
                            // log(tmp);
                            // log(tmp2);
                        }
                    }
            }
        }
        self.fire.retain(|elem| !elem.on_wall);
    }
    pub fn draw(&mut self){
        for i in &self.item {
            i.draw();
        }
        for b in &self.bomb {
            b.draw();
        }
        for b in &self.blocks {
            b.draw();
        }
        for f in &self.fire {
            f.draw();
        }
        for w in &self.walls {
            w.draw();
        }
        for p in &mut self.players {
            p.draw();
        }
    }
}

//fn get_bmp<T:Position>(obj_list: Vec<T>) -> Vec<i32> {
//    let mut result  = Vec::new();
//    let i:i32 = 0;
//    result = vec![i; 13];
//    for obj in obj_list {
//        let mut tmp = 1;
//        tmp << (15 -  (obj.x() - 25)/ 50);
//        result[ ((obj.y() -25) / 50) as usize ] = tmp;
//    }
//    result
//}
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}
