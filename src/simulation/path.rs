use bevy::prelude::*;
use pathfinding::prelude::Matrix;
use serde::{Deserialize, Serialize};

use crate::AppState;

// #[derive(Component, Serialize, Deserialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
// struct Edge;

#[derive(Component, Serialize, Deserialize, Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {}

pub struct PathPlugin;

impl Plugin for PathPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::InGame)))
            .add_system(cleanup.in_schedule(OnExit(AppState::InGame)));
    }
}

fn setup() {}

fn cleanup() {}
