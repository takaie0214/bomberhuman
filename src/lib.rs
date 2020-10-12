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

use self::game_state::GameState;
use self::geometry::Size;
use self::controller::Actions;

use std::os::raw::{c_double, c_int};

#[wasm_bindgen]
struct GameData {
    state: GameState,
    actions: Actions,
    //time_controller: TimeController<Pcg32Basic>
}

#[wasm_bindgen]
impl GameData {
    pub fn new(width: f64, height: f64) -> GameData {
        GameData {
            state: GameState::new(Size::new(width, height)),
            actions: Actions::default(),
            //time_controller: TimeController::new(Pcg32Basic::from_seed([42, 42]))
        }
    }
    pub fn update(&mut self, dt: f64) {
        self.state.update(dt, &self.actions);
    }
    pub fn get_objx(&self) -> f64{
        self.state.world.player.x()
    }

    pub fn get_objy(&self) -> f64{
        self.state.world.player.y()
    }

    pub fn toggle_move_up(&mut self, b: c_int) {
        self.actions.move_up = int_to_bool(b);
    }

    pub fn toggle_move_down(&mut self, b: c_int) {
        self.actions.move_down = int_to_bool(b);
    }

    pub fn toggle_move_right(&mut self, b: c_int) {
        self.actions.move_right = int_to_bool(b);
    }

    pub fn toggle_move_left(&mut self, b: c_int) {
        self.actions.move_left = int_to_bool(b);
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

