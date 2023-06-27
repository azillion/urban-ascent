mod cameras;
mod controls;
mod lights;
mod particles;
// mod physics;
mod render;
mod terrain;

use bevy::{
    prelude::*,
    window::{WindowMode, WindowPlugin},
};
// use bevy_editor_pls::prelude::*;
// use bevy_mod_picking::prelude::*;

struct UrbanAscentPlugin;

impl Plugin for UrbanAscentPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::DARK_GRAY))
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Urban Ascent".to_string(),
                    mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            }))
            // .add_plugin(EditorPlugin::default())
            // .add_plugins(DefaultPickingPlugins)
            // .add_plugin(particles::ParticlePlugin)
            .add_plugin(lights::LightsPlugin)
            .add_plugin(terrain::TerrainPlugin)
            .add_plugin(cameras::CameraPlugin)
            // .add_plugin(physics::PhysicsPlugin)
            .add_system(bevy::window::close_on_esc);
    }
}

fn main() {
    App::new().add_plugin(UrbanAscentPlugin).run();
}
