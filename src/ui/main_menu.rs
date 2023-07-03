use bevy::prelude::*;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::save::DEFAULT_SAVE_FOLDER;
use crate::{
    save::{GameState, Save},
    AppState,
};

use super::config::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const DISABLED_BUTTON: Color = Color::rgb(0.5, 0.5, 0.5);

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
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>, With<NewGameButton>),
    >,
    mut state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
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
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<ContinueGameButton>),
    >,
    mut state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                let file = get_most_recently_changed_file(DEFAULT_SAVE_FOLDER)
                    .expect("Failed to get most recently changed file");
                state.set(AppState::InGame);
                *game_state = game_state.load(file);
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

fn get_most_recently_changed_file(folder_path: &str) -> io::Result<PathBuf> {
    let folder = Path::new(folder_path);

    let mut most_recent_file: Option<PathBuf> = None;
    let mut most_recent_time: Option<SystemTime> = None;

    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let modified_time = metadata.modified()?;

        if let Some(current_time) = most_recent_time {
            if modified_time > current_time {
                most_recent_file = Some(entry.path());
                most_recent_time = Some(modified_time);
            }
        } else {
            most_recent_file = Some(entry.path());
            most_recent_time = Some(modified_time);
        }
    }

    most_recent_file
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No files found in the folder."))
}
