use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Component, Serialize, Deserialize, Debug)]
struct Person {
    first_name: u8,
    last_name: u8,
    age: u8,
    gender: u8,
}

pub struct PersonPlugin;

impl Plugin for PersonPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::InGame)))
            .add_system(cleanup.in_schedule(OnExit(AppState::InGame)));
    }
}

fn setup() {}

fn cleanup() {}
