mod manager;
mod person;
mod utils;

use manager::GameManager;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    log!("Hello Urban Ascent!");

    Ok(())
}

#[wasm_bindgen]
pub struct UrbanAscent {
    game_manager: Box<GameManager>,
}

#[wasm_bindgen]
impl UrbanAscent {
    #[wasm_bindgen(constructor)]
    pub fn new() -> UrbanAscent {
        let game_manager = GameManager::new();
        UrbanAscent {
            game_manager: Box::new(game_manager),
        }
    }

    pub fn tick(&mut self) {
        self.game_manager.tick();
    }

    #[wasm_bindgen(js_name = "newGame")]
    pub fn new_game(&mut self, town_name: String, player_name: String) {
        self.game_manager.new_game(town_name, player_name);
    }

    #[wasm_bindgen(js_name = "saveGame")]
    pub fn save_game(&self) {
        self.game_manager.save_game();
    }

    #[wasm_bindgen(js_name = "loadGame")]
    pub fn load_game(&mut self) -> Result<bool, JsValue> {
        self.game_manager.load_game()
    }

    #[wasm_bindgen(js_name = "getTownName")]
    pub fn get_town_name(&self) -> String {
        self.game_manager.get_town_name()
    }

    #[wasm_bindgen(js_name = "setTownName")]
    pub fn set_town_name(&mut self, town_name: String) {
        self.game_manager.set_town_name(town_name);
    }

    #[wasm_bindgen(js_name = "getPlayerName")]
    pub fn get_player_name(&self) -> String {
        self.game_manager.get_player_name()
    }

    #[wasm_bindgen(js_name = "setPlayerName")]
    pub fn set_player_name(&mut self, player_name: String) {
        self.game_manager.set_player_name(player_name);
    }

    #[wasm_bindgen(js_name = "getGridWidth")]
    pub fn get_grid_width(&self) -> usize {
        self.game_manager.get_game_data_ref().get_grid_width()
    }

    #[wasm_bindgen(js_name = "getGridHeight")]
    pub fn get_grid_height(&self) -> usize {
        self.game_manager.get_game_data_ref().get_grid_height()
    }
}
