mod game_state;
mod geometry;
mod controller;
mod models;
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use self::geometry::Size;
use self::controller::Actions;
use self::models::World;

use std::os::raw::{c_double, c_int};

#[wasm_bindgen]
struct GameState {
    // The world contains everything that needs to be drawn
    world: World,
    actions: Actions,
}

#[wasm_bindgen]
impl GameState {
    /// Returns a new `GameState` containing a `World` of the given `Size`
    pub fn new(width: f64, height: f64) -> GameState {
        GameState {
            world: World::new(Size::new(width, height)),
            actions: Actions::default(),
        }
    }
    pub fn update(&mut self, dt: f64){
        self.world.update(dt, &self.actions);
    }

    pub fn toggle_turn_up(&mut self, b: c_int) {
        self.actions.ArrowUp = int_to_bool(b)
    }

    pub fn toggle_turn_down(&mut self, b: c_int) {
        self.actions.ArrowDown = int_to_bool(b)
    }

    pub fn toggle_turn_right(&mut self, b: c_int) {
        self.actions.ArrowRight = int_to_bool(b)
    }

    pub fn toggle_turn_left(&mut self, b: c_int) {
        self.actions.ArrowLeft = int_to_bool(b)
    }

    pub fn get_objx(&self) -> f64 {
        self.world.player.x()
    }

    pub fn get_objy(&self) -> f64 {
        self.world.player.y()
    }
}

fn int_to_bool(i: c_int) -> bool {
    i != 0
}
//#[wasm_bindgen(module = "/mylib/foo.js")]
//#[wasm_bindgen(module = "/www/index.js")]
//#[wasm_bindgen]
//extern "C" {
//    fn name() -> String;
//    fn clear_screen();
//}

//pub extern "C" fn update(time: c_double) {
//    let data: &mut GameData = &mut DATA.lock().unwrap();
//    data.time_controller.update_seconds(time, &data.actions, &mut data.state);
//    CollisionsController::handle_collisions(&mut data.state);
//}
//
//fn int_to_bool(i: c_int) -> bool {
//    i != 0

//}

//#[wasm_bindgen(module = "/mylib/foo.js")]
//#[wasm_bindgen(module = "/www/index.js")]
//#[wasm_bindgen]
//extern "C" {
//    fn name() -> String;
//    fn clear_screen();
//}

//pub extern "C" fn update(time: c_double) {
//    let data: &mut GameData = &mut DATA.lock().unwrap();
//    data.time_controller.update_seconds(time, &data.actions, &mut data.state);
//    CollisionsController::handle_collisions(&mut data.state);
//}
//
//fn int_to_bool(i: c_int) -> bool {
//    i != 0
//}
//
//
//#[no_mangle]
//pub unsafe extern "C" fn draw() {
//    use geometry::{Advance, Position};
//    let data = &mut DATA.lock().unwrap();
//    let world = &data.state.world;
//
//    clear_screen();
//    draw_player(world.player.x(), world.player.y(), world.player.direction());
//}

