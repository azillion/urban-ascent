use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::{
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

pub struct DevUIPlugin;

impl Plugin for DevUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin)
            .add_system(setup.in_schedule(OnEnter(AppState::InGame)))
            .add_system(update_fps.in_set(OnUpdate(AppState::InGame)))
            .add_system(update_day_clock.in_set(OnUpdate(AppState::InGame)))
            .add_system(update_date_clock.in_set(OnUpdate(AppState::InGame)))
            .add_system(cleanup.in_schedule(OnExit(AppState::InGame)));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(DEFAULT_FONT);
    commands.spawn((
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
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Day: ",
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
        SimDayClockText,
    ));
    commands.spawn((
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
        text.sections[1].value = format!("{:?}\n", time_config.hour());
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

fn cleanup(mut commands: Commands, query: Query<Entity, With<DevUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
