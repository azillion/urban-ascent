use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    io::prelude::*,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::simulation::GameSpeed;

pub const DEFAULT_SAVE_FOLDER: &str = "saves";

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct GameState {
    city_name: String,
    mayor_name: String,
    money: u32,
    population: u32,
    seconds_in_day: f32, // 0.0 - 1440.0
    week: u8,            // 0 - 47
    month: u8,           // 0 - 11
    year: u32,
    game_speed: GameSpeed,
}

impl GameState {
    pub fn new(city_name: &str, mayor_name: &str) -> Self {
        GameState {
            city_name: city_name.to_string(),
            mayor_name: mayor_name.to_string(),
            ..Default::default()
        }
    }

    pub fn get_mut_ref(&mut self) -> &mut Self {
        self
    }

    pub fn load(&self, file: PathBuf) -> Self {
        let mut file = std::fs::File::open(file).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
        let mut state: GameState = serde_json::from_str(&contents).expect("Failed to parse file");
        state
    }

    pub fn set_city_name(&mut self, city_name: &str) {
        self.city_name = city_name.to_string();
    }

    pub fn set_mayor_name(&mut self, mayor_name: &str) {
        self.mayor_name = mayor_name.to_string();
    }

    pub fn set_game_speed(&mut self, game_speed: GameSpeed) {
        self.game_speed = game_speed;
    }

    pub fn set_money(&mut self, money: u32) {
        self.money = money;
    }

    pub fn set_population(&mut self, population: u32) {
        self.population = population;
    }

    pub fn set_seconds_in_day(&mut self, seconds_in_day: f32) {
        self.seconds_in_day = seconds_in_day;
    }

    pub fn set_week(&mut self, week: u8) {
        self.week = week;
    }

    pub fn set_month(&mut self, month: u8) {
        self.month = month;
    }

    pub fn set_year(&mut self, year: u32) {
        self.year = year;
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            city_name: "My City".to_string(),
            mayor_name: "Mayor".to_string(),
            money: 0,
            population: 0,
            seconds_in_day: 0.0,
            week: 0,
            month: 0,
            year: 2000,
            game_speed: GameSpeed::Normal,
        }
    }
}

pub trait Save {
    fn save(&self, path: &str);
}

impl Save for GameState {
    fn save(&self, path: &str) {
        let mut file_name = String::from(path);
        if path == "" {
            let current_time = SystemTime::now();
            let since_epoch = current_time
                .duration_since(UNIX_EPOCH)
                .expect("Failed to get current time");
            file_name = format!(
                "{}/{}-{}.urban-ascent",
                DEFAULT_SAVE_FOLDER,
                self.city_name,
                since_epoch.as_secs()
            );
        }
        let parent_dir = Path::new(DEFAULT_SAVE_FOLDER)
            .parent()
            .expect("Invalid file path");
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir).expect("Failed to create save directory");
        }
        let mut file = std::fs::File::create(file_name).unwrap();
        let json = serde_json::to_string(&self).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState::default());
    }
}
