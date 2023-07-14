use bevy::prelude::*;

use crate::save::GameState;
use crate::ui::config::*;
use crate::AppState;

#[derive(Component)]
struct Bottom;

#[derive(Component)]
struct StraightRoadButton;

#[derive(Component)]
struct StraightRoadButtonText;

pub struct BottomPlugin;

impl Plugin for BottomPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::InGame)))
            .add_system(straight_road_button_system.in_set(OnUpdate(AppState::InGame)))
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
                        bottom: Val::Px(200.0),
                        left: Val::Px(0.0),
                        ..Default::default()
                    },
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::FlexEnd,
                    ..Default::default()
                },
                ..Default::default()
            },
            Bottom,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    },
                    StraightRoadButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle {
                            text: Text::from_section(
                                "S",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: DEFAULT_FONT_SIZE,
                                    color: WHITE,
                                },
                            ),
                            ..Default::default()
                        },
                        StraightRoadButtonText,
                    ));
                });
        });
}

fn straight_road_button_system(
    commands: Commands,
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), With<StraightRoadButton>>,
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

fn cleanup(mut commands: Commands, query: Query<Entity, With<Bottom>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
