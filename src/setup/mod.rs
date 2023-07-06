mod cameras;
mod controls;
mod lights;
mod particles;
mod physics;

use bevy::prelude::*;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(lights::LightsPlugin)
            .add_plugin(cameras::CameraPlugin)
            .add_plugin(controls::ControlsPlugin);
    }
}
