mod dev;
mod top;
mod top_right;

use bevy::prelude::*;

pub struct InGameUIPlugin;

impl Plugin for InGameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(top::TopPlugin);
    }
}
