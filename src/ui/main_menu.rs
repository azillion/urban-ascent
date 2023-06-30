use bevy::prelude::*;

use crate::AppState;

use super::config::*;

#[derive(Component)]
struct MainMenu;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(menu.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(cleanup_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "Urban Ascent",
                TextStyle {
                    font: asset_server.load(DEFAULT_FONT),
                    font_size: DEFAULT_TITLE_FONT_SIZE,
                    color: DEFAULT_FONT_COLOR,
                },
            ),
            ..Default::default()
        },
        MainMenu,
    ));
}

fn menu(
    mut state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<MainMenu>),
    >,
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

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
