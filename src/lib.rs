mod utils;

use legion::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Game {
    world: World,
    resources: Resources,
    schedule: Schedule,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        let world = World::default();
        let resources = Resources::default();

        // construct a schedule (you should do this on init)
        let schedule = Schedule::builder()
            // .add_system(update_positions_system())
            .build();

        Game {
            world,
            resources,
            schedule,
        }
    }

    // fn world(&self) -> &World {
    //     &self.world
    // }

    // fn resources(&self) -> &Resources {
    //     &self.resources
    // }

    // fn schedule(&self) -> &Schedule {
    //     &self.schedule
    // }

    pub fn tick(&mut self) {
        log!("tick");

        self.schedule.execute(&mut self.world, &mut self.resources);
    }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    log!("Hello Urban Ascent!");

    // TODO load saved game from local storage
    // if no saved game, start new game

    // construct a schedule (you should do this on init)
    // let mut schedule = Schedule::builder()
    //     .add_system(update_positions_system())
    //     .build();

    Ok(())
}
