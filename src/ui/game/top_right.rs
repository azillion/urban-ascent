use bevy::prelude::*;

use crate::save::{get_most_recently_changed_file, GameState, Save, DEFAULT_SAVE_FOLDER};
use crate::ui::config::*;
use crate::AppState;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const DISABLED_BUTTON: Color = Color::rgb(0.5, 0.5, 0.5);

#[derive(Component)]
struct TopRight;

#[derive(Component)]
struct SaveButton;

#[derive(Component)]
struct SaveButtonText;

pub struct TopRightPlugin;

impl Plugin for TopRightPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::InGame)))
            .add_system(save_button_system.in_set(OnUpdate(AppState::InGame)))
            .add_system(cleanup.in_schedule(OnExit(AppState::InGame)));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(DEFAULT_FONT);
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(300.0), Val::Undefined),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Px(0.0),
                        right: Val::Px(0.0),
                        ..Default::default()
                    },
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::FlexEnd,
                    ..Default::default()
                },
                ..Default::default()
            },
            TopRight,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(100.0), Val::Px(50.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    },
                    SaveButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle {
                            text: Text::from_section(
                                "Save",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: DEFAULT_FONT_SIZE,
                                    color: WHITE,
                                },
                            ),
                            ..Default::default()
                        },
                        SaveButtonText,
                    ));
                });
        });
}

fn save_button_system(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), With<SaveButton>>,
    game_state: Res<GameState>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                if let Some(file_path) = get_most_recently_changed_file(DEFAULT_SAVE_FOLDER)
                    .expect("No save file found")
                    .to_str()
                {
                    game_state.save(file_path);
                } else {
                    println!("No save file found");
                }
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

fn cleanup(mut commands: Commands, query: Query<Entity, With<TopRight>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
