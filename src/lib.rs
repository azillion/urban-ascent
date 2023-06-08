mod manager;
mod person;
mod urban_ascent;
mod utils;

use wasm_bindgen::prelude::*;

///////////////////////////
//   ___ ___      __                    _______                        __
//  |   Y   .----|  |--.---.-.-----.   |   _   .-----.----.-----.-----|  |_
//  |.  |   |   _|  _  |  _  |     |   |.  1   |__ --|  __|  -__|     |   _|
//  |.  |   |__| |_____|___._|__|__|   |.  _   |_____|____|_____|__|__|____|
//  |:  1   |                          |:  |   |
//  |::.. . |                          |::.|:. |
//  `-------'                          `--- ---'
///////////////////////////

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
