use crate::log;

use legion::*;
use pathfinding::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::window;

const MAX_LENGTH: usize = 255;

pub struct GameManager {
    storage: web_sys::Storage,
    game_save_key: &'static str,
    game_data: Option<GameData>,
    grid: Option<Grid>,
}

impl GameManager {
    #[must_use]
    pub fn new() -> Self {
        let game_save_key = "game-0.1.0";

        let storage = window().unwrap().local_storage().unwrap().unwrap_throw();

        let mut game_manager = Self {
            storage,
            game_save_key,
            game_data: None,
            grid: None,
        };
        game_manager.setup_database();
        game_manager
    }

    fn setup_database(&mut self) {}

    pub fn tick(&mut self) {
        if self.game_data.is_none() || self.grid.is_none() {
            return;
        }
    }

    pub fn new_game(&mut self, town_name: String, player_name: String) {
        log!("{:?} - {:?}", town_name, player_name);
        self.game_data = Some(GameData::new(town_name, player_name));
        self.save_game();
    }

    pub fn save_game(&self) {
        // let window = window().unwrap();
        // window.alert_with_message("Saving game").unwrap();
        let game_save: &str = &serde_json::to_string(&self.game_data).unwrap();
        if let Err(err) = self.storage.set_item(self.game_save_key, game_save) {
            log!("Error storing data: {:?}", err);
        }
    }

    pub fn load_game(&mut self) -> Result<bool, JsValue> {
        if let Ok(value) = self.storage.get_item(self.game_save_key) {
            if let Some(value) = value {
                log!("Loading game: {:?}", value);
                let game_save: GameData = serde_json::from_str(&value).unwrap();
                self.game_data = Some(game_save);
                if let Some(game_data) = &self.game_data {
                    let grid_size = game_data.get_grid_size();
                    self.grid = Some(Grid::new(grid_size.0, grid_size.1));
                    return Ok(true);
                }
                return Ok(false);
            }
        } else {
            log!("Error retrieving data");
            return Err(JsValue::from_str("Error retrieving data"));
        }

        Ok(false)
    }

    pub fn get_game_data_ref(&self) -> &GameData {
        self.game_data.as_ref().unwrap()
    }

    pub fn get_town_name(&self) -> String {
        if let Some(game_data) = &self.game_data {
            return game_data.town_name.clone();
        }
        String::from("No town name")
    }

    pub fn set_town_name(&mut self, town_name: String) {
        if let Some(game_data) = &mut self.game_data {
            game_data.town_name = town_name.chars().take(MAX_LENGTH).collect();
        }
    }

    pub fn get_player_name(&self) -> String {
        if let Some(game_data) = &self.game_data {
            return game_data.player_name.clone();
        }
        String::from("No player name")
    }

    pub fn set_player_name(&mut self, player_name: String) {
        if let Some(game_data) = &mut self.game_data {
            game_data.player_name = player_name.chars().take(MAX_LENGTH).collect();
        }
    }
}

pub fn log_game_data(game_data: &GameData) {
    log!("{:?}", game_data);
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameData {
    town_name: String,
    player_name: String,
    grid_height: usize,
    grid_width: usize,
    world_data_serialized: Option<String>,
}

impl GameData {
    #[must_use]
    pub fn new(town_name: String, player_name: String) -> Self {
        let mut registry = Registry::<String>::default();
        // registry.register::<Position>("position".to_string());
        // registry.register::<f32>("f32".to_string());
        // registry.register::<bool>("bool".to_string());
        // let mut world = World::default();
        // let world_data: serialize::SerializableWorld<_, _, _> = world.as_serializable();
        Self {
            town_name: town_name.chars().take(MAX_LENGTH).collect(),
            player_name: player_name.chars().take(MAX_LENGTH).collect(),
            grid_height: 1000,
            grid_width: 1000,
            world_data_serialized: None,
            // world: World::default(),
        }
    }

    pub fn get_grid_size(&self) -> (usize, usize) {
        (self.grid_width, self.grid_height)
    }

    pub fn get_grid_width(&self) -> usize {
        self.grid_width
    }

    pub fn get_grid_height(&self) -> usize {
        self.grid_height
    }
}
