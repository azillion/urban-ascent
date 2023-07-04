use bevy::prelude::*;

use crate::AppState;

use super::dev::DevUIPlugin;
use super::top_right::TopRightPlugin;

#[derive(Component)]
struct Top;

pub struct TopPlugin;

impl Plugin for TopPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DevUIPlugin)
            .add_plugin(TopRightPlugin)
            .add_system(cleanup.in_schedule(OnExit(AppState::InGame)));
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<Top>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
