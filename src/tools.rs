use bevy::prelude::*;

use crate::AppState;

#[derive(Component, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Tools {
    Pan,
    Select,
    Bulldoze,
    Place,
    AddZone,
    AddPath,
    ReplacePath,
    RemovePath,
}

#[derive(Resource)]
pub struct ToolsState {
    pub current: Tools,
    pub previous: Tools,
}

impl Default for ToolsState {
    fn default() -> Self {
        Self {
            current: Tools::Pan,
            previous: Tools::Pan,
        }
    }
}

impl ToolsState {
    pub fn set(&mut self, tool: Tools) {
        self.previous = self.current;
        self.current = tool;
    }

    pub fn reset(&mut self) {
        self.previous = self.current;
        self.current = Tools::Pan;
    }
}

#[derive(Resource)]
pub struct ToolsKeyCodeConfig {
    pub pan: KeyCode,
    pub select: KeyCode,
    pub bulldoze: KeyCode,
    pub place: KeyCode,
    pub add_zone: KeyCode,
    pub add_path: KeyCode,
    pub replace_path: KeyCode,
    pub remove_path: KeyCode,
}

impl Default for ToolsKeyCodeConfig {
    fn default() -> Self {
        Self {
            pan: KeyCode::P,
            select: KeyCode::O,
            bulldoze: KeyCode::B,
            place: KeyCode::N,
            add_zone: KeyCode::Z,
            add_path: KeyCode::R,
            replace_path: KeyCode::T,
            remove_path: KeyCode::Y,
        }
    }
}

pub struct ToolsPlugin;

impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ToolsKeyCodeConfig::default())
            .insert_resource(ToolsState::default())
            .add_system(setup.in_schedule(OnEnter(AppState::InGame)))
            .add_system(cleanup.in_schedule(OnExit(AppState::InGame)));
    }
}

fn setup(mut commands: Commands) {}

fn cleanup(mut commands: Commands, query: Query<Entity>) {}
// 	for entity in query.iter() {
// 		commands.entity(entity).despawn_recursive();
// 	}
// }
