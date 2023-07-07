mod consumers;
mod logistics;
mod person;
mod producers;
mod resources;
mod terrain;
mod time;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub use time::TimeConfig;
use time::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Resource, Serialize, Deserialize)]
pub enum GameSpeed {
    Paused,
    Normal,
    Fast,
    Faster,
}

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameSpeed::Normal)
            .add_plugin(terrain::TerrainPlugin)
            .add_plugin(TimePlugin)
            .add_plugin(resources::ResourcesPlugin)
            .add_plugin(person::PersonPlugin);
    }
}

pub fn run_if_not_paused(speed: Res<GameSpeed>) -> bool {
    match *speed {
        GameSpeed::Paused => false,
        _ => true,
    }
}
