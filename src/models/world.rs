use crate::geometry::{Point, Size};
use rand::Rng;
use pcg_rand::Pcg32Basic;
use rand::SeedableRng;
use crate::models::{Player, Bomb, Wall,  Block, Fire};
use std::collections::HashMap;
use crate::controller::{Controller, Event};

/// A model that contains the other models and renders them
pub struct World {
    pub players: Vec<Player>,
    pub bomb: Vec<Bomb>,
    pub walls: Vec<Wall>,
    pub blocks: Vec<Block>,
    pub fire: Vec<Fire>,
    pub size: Size,
    pub event: Vec<Event>,
}

impl World {
    pub fn new(size: Size) -> World {
        let mut players = Vec::new();

        let id1 = 101;
        let point1 = Point::new(75, 75);
        let controller1 = Controller::new("up1","down1","right1","left1", "a1");
        players.push(Player::new(id1, point1, controller1));

        let id2 = 102;
        let point2 = Point::new(675, 575);
        let controller2 = Controller::new("up2","down2","right2","left2", "a2");
        players.push(Player::new(id2, point2, controller2));

        let id3 = 103;
        let point3 = Point::new(675, 75);
        let controller3 = Controller::new("up3","down3","right3","left3", "a3");
        players.push(Player::new(id3, point3, controller3));

        let id4 = 104;
        let point4 = Point::new(75, 575);
        let controller4 = Controller::new("up4","down4","right4","left4", "a4");
        players.push(Player::new(id4, point4, controller4));

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
        for (index1, val1) in block_bmp.iter().enumerate(){
            for (index2,val2) in val1.iter().enumerate(){
                if *val2 == 1 && rng.gen_range(0, 9) > 0 {
                    blocks.push(Block::new(Point::new(50 * index2 as i32 + 25, 50 * index1 as i32 + 25)));
                }
            }
        }

        let fire = Vec::new();

        World {
            players: players,
            bomb: vec![],
            walls: walls,
            blocks: blocks,
            fire: fire,
            size: size,
            event: vec![],
        }
    }

    pub fn update(&mut self, dt: f64, actions: &HashMap<String, bool>) {
        for p in &mut self.players {
            p.update(dt, actions, &mut self.event);
        }
        for b in &mut self.bomb {
            b.update(dt,  &mut self.event);
        }
        for b in &mut self.blocks {
            b.update();
        }
        for f in &mut self.fire {
            f.update(dt,  &mut self.event);
        }
        while self.event.len() != 0 {
            match self.event.pop() {
                None => (),
                Some(event) =>
                    match event {
                        Event::SetBomb{id,x,y} => {
                            for b in &self.bomb{
                                if b.point.x == x && b.point.y == y {
                                    return ();
                                }
                            }
                            self.bomb.push(Bomb::new(id,x,y));
                        },
                        Event::Explosion{id, x, y, dir} => {
                            self.bomb.retain(|elem| elem.id != id);
                            self.fire.push(Fire::new(id, x, y, dir));
                        }
                        Event::Disappearance{id} => {
                            self.fire.retain(|elem| elem.id != id);
                        }
                    }
            }
        }
    }
    pub fn draw(&mut self){
        for p in &mut self.players {
            p.draw();
        }
        for w in &self.walls {
            w.draw();
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


