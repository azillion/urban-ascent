use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Component, Serialize, Deserialize, Debug)]
struct Resource;

#[derive(Component, Serialize, Deserialize, Debug)]
struct Commuter;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::InGame)))
            .add_system(cleanup.in_schedule(OnExit(AppState::InGame)));
    }
}

// load resources from game state
fn setup() {}

// save resources to game state
fn cleanup() {}
