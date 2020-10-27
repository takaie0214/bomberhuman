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
use std::collections::HashMap;

use std::os::raw::{c_double, c_int};

#[wasm_bindgen]
struct GameState {
    // The world contains everything that needs to be drawn
    world: World,
    actions: HashMap<String, bool>,
}

#[wasm_bindgen]
impl GameState {
    /// Returns a new `GameState` containing a `World` of the given `Size`
    pub fn new(width: f64, height: f64) -> GameState {
        GameState {
            world: World::new(Size::new(width, height)),
            actions: HashMap::new(),
        }
    }
    pub fn update(&mut self, dt: f64){
        self.world.update(dt, &self.actions);
    }
    pub fn draw(&self){
        clear_screen();
        self.world.draw();
    }

    pub fn processKey(&mut self, key: &str, b: c_int) {
        match key {
            "ArrowUp"    => self.actions.insert(String::from("ArrowUp"), int_to_bool(b)),
            "ArrowDown"  => self.actions.insert(String::from("ArrowDown"), int_to_bool(b)),
            "ArrowRight" => self.actions.insert(String::from("ArrowRight"), int_to_bool(b)),
            "ArrowLeft"  => self.actions.insert(String::from("ArrowLeft"), int_to_bool(b)),
            " "          => self.actions.insert(String::from("Space"), int_to_bool(b)),
            "w"          => self.actions.insert(String::from("w"), int_to_bool(b)),
            "s"          => self.actions.insert(String::from("s"), int_to_bool(b)),
            "a"          => self.actions.insert(String::from("a"), int_to_bool(b)),
            "d"          => self.actions.insert(String::from("d"), int_to_bool(b)),
            "m"          => self.actions.insert(String::from("m"), int_to_bool(b)),
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

