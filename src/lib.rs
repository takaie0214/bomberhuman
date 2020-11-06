mod game_state;
mod geometry;
mod controller;
mod models;
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.  #[cfg(feature = "wee_alloc")]
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use self::geometry::Size;
use self::controller::Actions;
use self::models::World;
use self::controller::Collision;
use std::collections::HashMap;

use std::os::raw::{c_double, c_int};

#[wasm_bindgen] struct GameState {
    // The world contains everything that needs to be drawn
    world: World,
    actions: HashMap<String, bool>,
    // collision: Collision
}

#[wasm_bindgen]
impl GameState {
    /// Returns a new `GameState` containing a `World` of the given `Size`
    pub fn new(width: i32, height: i32) -> GameState {
        GameState {
            world: World::new(Size::new(width, height)),
            actions: HashMap::new(),
            // collision: Collision::new()
        }
    }
    pub fn update(&mut self, dt: f64){
        self.world.update(dt, &self.actions);
        Collision::collision_with(dt,&self.actions,&mut self.world);
    }
    pub fn draw(&mut self){
        clear_screen();
        self.world.draw();
    }

    pub fn processKey(&mut self, key: &str, b: c_int) {
        match key {
            "ArrowUp"    => self.actions.insert(String::from("up1"), int_to_bool(b)),
            "ArrowDown"  => self.actions.insert(String::from("down1"), int_to_bool(b)),
            "ArrowRight" => self.actions.insert(String::from("right1"), int_to_bool(b)),
            "ArrowLeft"  => self.actions.insert(String::from("left1"), int_to_bool(b)),
            " "          => self.actions.insert(String::from("a1"), int_to_bool(b)),
            "w"          => self.actions.insert(String::from("up2"), int_to_bool(b)),
            "s"          => self.actions.insert(String::from("down2"), int_to_bool(b)),
            "d"          => self.actions.insert(String::from("right2"), int_to_bool(b)),
            "a"          => self.actions.insert(String::from("left2"), int_to_bool(b)),
            "m"          => self.actions.insert(String::from("a2"), int_to_bool(b)),
            "1"          => self.actions.insert(String::from("up3"), int_to_bool(b)),
            "2"          => self.actions.insert(String::from("down3"), int_to_bool(b)),
            "3"          => self.actions.insert(String::from("right3"), int_to_bool(b)),
            "4"          => self.actions.insert(String::from("left3"), int_to_bool(b)),
            "5"          => self.actions.insert(String::from("a4"), int_to_bool(b)),
            "6"          => self.actions.insert(String::from("up4"), int_to_bool(b)),
            "7"          => self.actions.insert(String::from("down4"), int_to_bool(b)),
            "8"          => self.actions.insert(String::from("right4"), int_to_bool(b)),
            "9"          => self.actions.insert(String::from("left4"), int_to_bool(b)),
            "0"          => self.actions.insert(String::from("a4"), int_to_bool(b)),
                       _ => None,
        };
    }
}

fn int_to_bool(i: c_int) -> bool {
    i != 0
}


#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen(module = "/src/javascript/canvas.js")]
extern "C" {
    pub fn clear_screen();
}

