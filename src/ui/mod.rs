mod config;
mod game;
mod main_menu;
mod state;

use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(main_menu::MainMenuPlugin)
            .add_plugin(game::InGameUIPlugin);
    }
}
