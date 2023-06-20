mod cameras;
mod lights;
mod render;
mod terrain;

use bevy::prelude::*;
// use bevy_editor_pls::prelude::*;
// use bevy_mod_picking::prelude::*;

struct UrbanAscentPlugin;

impl Plugin for UrbanAscentPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::DARK_GRAY))
            .add_plugins(DefaultPlugins)
            // .add_plugin(EditorPlugin::default())
            // .add_plugins(DefaultPickingPlugins)
            .add_plugin(cameras::CameraPlugin)
            .add_plugin(lights::LightsPlugin)
            .add_plugin(terrain::TerrainPlugin)
            .add_system(bevy::window::close_on_esc);
    }
}

fn main() {
    App::new().add_plugin(UrbanAscentPlugin).run();
}
