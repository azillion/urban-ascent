use bevy::prelude::*;

use crate::save::DEFAULT_SAVE_FOLDER;
use crate::simulation::TimeConfig;
use crate::{
    save::{GameState, Save},
    AppState,
};

use super::config::*;
use crate::save::get_most_recently_changed_file;

#[derive(Component)]
struct NewGameButton;

#[derive(Component)]
struct ContinueGameButton;

#[derive(Component)]
struct MainMenu;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_menu.in_schedule(OnEnter(AppState::MainMenu)))
            // .add_system(menu.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(new_game_button_system.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(continue_game_button_enabled_system.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(continue_game_button_system.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(cleanup_menu.in_schedule(OnExit(AppState::MainMenu)));
        // .add_system(continue_game_button_system.in_set(OnUpdate);
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::width(Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(300.0), Val::Px(100.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    NewGameButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "New Game",
                        TextStyle {
                            font: asset_server.load(DEFAULT_BOLD_FONT),
                            font_size: DEFAULT_TITLE_FONT_SIZE,
                            color: DEFAULT_FONT_COLOR,
                        },
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(300.0), Val::Px(100.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: DISABLED_BUTTON.into(),
                        ..default()
                    },
                    ContinueGameButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Continue Game",
                        TextStyle {
                            font: asset_server.load(DEFAULT_BOLD_FONT),
                            font_size: DEFAULT_TITLE_FONT_SIZE,
                            color: DEFAULT_FONT_COLOR,
                        },
                    ));
                });
        });
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn new_game_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<NewGameButton>),
    >,
    mut state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                state.set(AppState::InGame);
                *game_state = GameState::default();
                game_state.save("");
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn continue_game_button_enabled_system(
    mut query: Query<&mut BackgroundColor, With<ContinueGameButton>>,
) {
    for mut color in &mut query {
        if color.0 == DISABLED_BUTTON.into() {
            let has_previous_save = get_most_recently_changed_file(DEFAULT_SAVE_FOLDER).is_ok();
            if has_previous_save {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn continue_game_button_system(
    // world: &mut World,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<ContinueGameButton>),
    >,
    mut state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<GameState>,
    mut time_config: ResMut<TimeConfig>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                let file = get_most_recently_changed_file(DEFAULT_SAVE_FOLDER)
                    .expect("Failed to get most recently changed file");
                let loaded_game_state = GameState::load(file).expect("Failed to load game state");
                game_state.update(loaded_game_state);
                println!("Loaded game state: {:?}", game_state);
                time_config.update_from_game_state(&game_state);
                state.set(AppState::InGame);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
