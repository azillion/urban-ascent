mod atoms;
mod config;
mod molecules;
mod organisms;
mod state;

use bevy::prelude::*;

use config::*;

use crate::AppState;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct MainMenuText;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_system(setup_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(menu.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(cleanup_menu.in_schedule(OnExit(AppState::MainMenu)))
            .add_startup_system(setup_fps)
            .add_system(update_fps);
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "Urban Ascent",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
            ),
            ..Default::default()
        },
        MainMenuText,
    ));
}

fn menu(
    mut state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    for interaction in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                state.set(AppState::InGame);
            }
            _ => {}
        }
    }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MainMenuText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_fps(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "FPS: 0.00",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            ),
            ..Default::default()
        },
        FpsText,
    ));
}

fn update_fps(
    time: Res<Time>,
    mut query: Query<&mut Text, With<FpsText>>,
    current_state: Res<State<AppState>>,
    mut state: ResMut<NextState<AppState>>,
    mut fps_history: Local<Vec<f32>>,
) {
    if current_state.0 == AppState::MainMenu {
        state.set(AppState::InGame);
    }
    let mut current_fps = 1.0 / time.delta_seconds();
    if current_fps.is_nan() {
        current_fps = 0.0;
    }
    fps_history.push(current_fps);
    if fps_history.len() > 100 {
        fps_history.remove(0);
    }
    let historical_fps: f32 = fps_history.iter().sum::<f32>() / fps_history.len() as f32;

    let fps = if historical_fps.is_infinite() || historical_fps.is_nan() {
        current_fps
    } else {
        historical_fps
    };
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("FPS: {:.0}", fps);
    }
}
