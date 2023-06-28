mod time;

use bevy::prelude::*;

use time::*;

#[derive(Debug, PartialEq, Eq, Resource)]
pub enum GameSpeed {
    Paused,
    Normal,
    Fast,
    Faster,
}

fn run_if_not_paused(speed: Res<GameSpeed>) -> bool {
    match *speed {
        GameSpeed::Paused => false,
        _ => true,
    }
}

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        let game_running_system_set = (update_time).run_if(run_if_not_paused);

        app.insert_resource(GameSpeed::Normal)
            .add_system(game_running_system_set);
    }
}
