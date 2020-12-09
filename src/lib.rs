mod geometry;
mod controller;
mod models;
//mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.  #[cfg(feature = "wee_alloc")]
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use self::geometry::Size;
use self::models::World;
use self::controller::{Controller, Collision, KeyType};

use std::os::raw::c_int;

#[wasm_bindgen] struct GameState {
    // The world contains everything that needs to be drawn
    world: World,
    controllers: Vec<Controller>,
}

#[wasm_bindgen]
impl GameState {
    /// Returns a new `GameState` containing a `World` of the given `Size`
    pub fn new(width: i32, height: i32) -> GameState {
        GameState {
            world: World::new(Size::new(width, height)),
            controllers: vec![Controller::new();4],
        }
    }
    pub fn update(&mut self, dt: f64){
        self.world.update(dt, &self.controllers);
        Collision::collision_with(dt, &self.controllers, &mut self.world);
    }
    pub fn draw(&mut self){
        clear_screen();
        self.world.draw();
    }

    pub fn processkey(&mut self, key: &str, b: bool) {
        let state = b;
        match key {
            //keymap for Player1
            "ArrowUp"    => self.controllers[0].up       = state,
            "ArrowDown"  => self.controllers[0].down     = state,
            "ArrowRight" => self.controllers[0].right    = state,
            "ArrowLeft"  => self.controllers[0].left     = state,
            " "          => self.controllers[0].button1  = state,

            //keymap for Player2
            "w"          => self.controllers[1].up       = state,
            "s"          => self.controllers[1].down     = state,
            "d"          => self.controllers[1].right    = state,
            "a"          => self.controllers[1].left     = state,
            "m"          => self.controllers[1].button1  = state,
            "n"          => self.controllers[1].button2  = state,

            //keymap for Player3
            "1"          => self.controllers[2].up       = state,
            "2"          => self.controllers[2].down     = state,
            "3"          => self.controllers[2].right    = state,
            "4"          => self.controllers[2].left     = state,
            "5"          => self.controllers[2].button1  = state,

            //keymap for Player4
            "6"          => self.controllers[3].up       = state,
            "7"          => self.controllers[3].down     = state,
            "8"          => self.controllers[3].right    = state,
            "9"          => self.controllers[3].left     = state,
            "0"          => self.controllers[3].button1  = state,

            _            => (),
        };
    }
    pub fn processcontroller(&mut self, order: c_int, key: KeyType, state: bool){
        match key {
            KeyType::Up     => self.controllers[order as usize].up       = state,
            KeyType::Down   => self.controllers[order as usize].down     = state,
            KeyType::Right  => self.controllers[order as usize].right    = state,
            KeyType::Left   => self.controllers[order as usize].left     = state,
            KeyType::Button1=> self.controllers[order as usize].button1  = state,
            // KeyType::Button2=> self.controllers[order as usize].button2  = state,
        }
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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
