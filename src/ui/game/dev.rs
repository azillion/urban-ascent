use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::{
    setup::Sun,
    simulation::TimeConfig,
    ui::config::{DEFAULT_FONT, DEFAULT_FONT_SIZE, WHITE},
    AppState,
};

#[derive(Component)]
struct DevUI;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct SimDayClockText;

#[derive(Component)]
struct SimDateClockText;

#[derive(Component)]
struct SunPositionText;

pub struct DevUIPlugin;

impl Plugin for DevUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin)
            .add_system(setup.in_schedule(OnEnter(AppState::InGame)))
            .add_system(update_fps.in_set(OnUpdate(AppState::InGame)))
            .add_system(update_day_clock.in_set(OnUpdate(AppState::InGame)))
            .add_system(update_date_clock.in_set(OnUpdate(AppState::InGame)))
            // .add_system(update_sun_pos_text.in_set(OnUpdate(AppState::InGame)))
            .add_system(cleanup.in_schedule(OnExit(AppState::InGame)));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(DEFAULT_FONT);
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    max_size: Size::new(Val::Px(300.0), Val::Undefined),
                    // horizontally center child text
                    justify_content: JustifyContent::FlexStart,
                    // vertically center child text
                    align_items: AlignItems::FlexStart,
                    // border: UiRect::all(Val::Px(2.0)),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            },
            DevUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "FPS: 0.0\n",
                        TextStyle {
                            font: font.clone(),
                            font_size: DEFAULT_FONT_SIZE,
                            color: WHITE,
                        },
                    ),
                    ..Default::default()
                },
                FpsText,
                DevUI,
            ));
            parent.spawn((
                TextBundle::from_sections([TextSection::from_style(TextStyle {
                    font: font.clone(),
                    font_size: DEFAULT_FONT_SIZE,
                    color: WHITE,
                })]),
                DevUI,
                SimDayClockText,
            ));
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        "Date: ",
                        TextStyle {
                            font: font.clone(),
                            font_size: DEFAULT_FONT_SIZE,
                            color: WHITE,
                        },
                    ),
                    TextSection::from_style(TextStyle {
                        font: font.clone(),
                        font_size: DEFAULT_FONT_SIZE,
                        color: WHITE,
                    }),
                ]),
                DevUI,
                SimDateClockText,
            ));
            // parent.spawn((
            //     TextBundle::from_sections([
            //         TextSection::new(
            //             "Sun Pos: ",
            //             TextStyle {
            //                 font: font.clone(),
            //                 font_size: DEFAULT_FONT_SIZE,
            //                 color: WHITE,
            //             },
            //         ),
            //         TextSection::from_style(TextStyle {
            //             font: font.clone(),
            //             font_size: DEFAULT_FONT_SIZE,
            //             color: WHITE,
            //         }),
            //         TextSection::from_style(TextStyle {
            //             font: font.clone(),
            //             font_size: DEFAULT_FONT_SIZE,
            //             color: WHITE,
            //         }),
            //         TextSection::from_style(TextStyle {
            //             font: font.clone(),
            //             font_size: DEFAULT_FONT_SIZE,
            //             color: WHITE,
            //         }),
            //     ]),
            //     DevUI,
            //     SunPositionText,
            // ));
        });
}

fn update_fps(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        // text.sections[0].value = format!("FPS: {:.0}", fps);
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[0].value = format!("FPS: {value:.1}\n");
            }
        }
    }
}

fn update_day_clock(
    time_config: Res<TimeConfig>,
    mut query: Query<&mut Text, With<SimDayClockText>>,
) {
    for mut text in query.iter_mut() {
        let minute_string = if time_config.minute() < 10 {
            format!("0{}", time_config.minute())
        } else {
            format!("{}", time_config.minute())
        };
        text.sections[0].value = format!("{}:{}\n", time_config.hour(), minute_string);
    }
}

fn update_date_clock(
    time_config: Res<TimeConfig>,
    mut query: Query<&mut Text, With<SimDateClockText>>,
) {
    for mut text in query.iter_mut() {
        text.sections[1].value = format!(
            "{:?}/{:?}/{:?}\n",
            time_config.week(),
            time_config.month(),
            time_config.year()
        );
    }
}

fn update_sun_pos_text(
    time: Res<TimeConfig>,
    sun_query: Query<&Transform, With<Sun>>,
    mut query: Query<&mut Text, With<SunPositionText>>,
) {
    let sun_transform = sun_query.get_single().unwrap();
    let sun_pos = sun_transform.translation;
    let sun_rot = sun_transform.rotation;

    for mut text in query.iter_mut() {
        text.sections[1].value = format!(
            "(x: {:.2}, y: {:.2}, z: {:.2})\n",
            sun_pos.x, sun_pos.y, sun_pos.z,
        );
        text.sections[2].value =
            format!("({:.2}, {:.2}, {:.2})\n", sun_rot.x, sun_rot.y, sun_rot.z,);
        text.sections[3].value = format!("{}%\n", time.daylight_amount());
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<DevUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
