#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod render;
mod save;
mod setup;
mod simulation;
mod tools;
mod ui;

use bevy::{
    prelude::*,
    window::{WindowMode, WindowPlugin},
};
// use bevy_editor_pls::prelude::*;
// use bevy_mod_picking::prelude::*;

const WINDOW_TITLE: &str = "Urban Ascent";

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    PauseMenu,
}

struct UrbanAscentPlugin;

impl Plugin for UrbanAscentPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .insert_resource(ClearColor(Color::DARK_GRAY))
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: WINDOW_TITLE.to_string(),
                    mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            }))
            // .add_plugin(EditorPlugin::default())
            // .add_plugins(DefaultPickingPlugins)
            .add_plugin(setup::SetupPlugin)
            .add_plugin(simulation::SimulationPlugin)
            .add_plugin(ui::UIPlugin)
            .add_plugin(save::SavePlugin)
            .add_system(bevy::window::close_on_esc);
    }
}

fn main() {
    App::new().add_plugin(UrbanAscentPlugin).run();
}
