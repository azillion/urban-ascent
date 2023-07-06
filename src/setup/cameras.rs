use bevy::{
    core_pipeline::{
        bloom::{BloomCompositeMode, BloomPrefilterSettings, BloomSettings},
        fxaa::Fxaa,
        tonemapping::Tonemapping,
    },
    prelude::*,
};
use bevy_atmosphere::prelude::*;
use bevy_hanabi::prelude::*;

use super::controls::PanOrbitCameraControls;

#[derive(Component)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_main_camera)
            .add_system(toggle_fog_system);
        // .add_system(move_particles_with_camera)
        // .add_system(update_bloom_settings);
    }
}

fn setup_main_camera(mut commands: Commands) {
    let translation = Vec3::new(0.0, 5.0, 15.0);
    let radius = translation.length();

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        FogSettings {
            color: Color::rgba(0.1, 0.2, 0.4, 1.0),
            directional_light_color: Color::rgba(1.0, 0.95, 0.75, 0.5),
            directional_light_exponent: 15.0,
            falloff: FogFalloff::from_visibility_colors(
                1000.0, // distance in world units up to which objects retain visibility (>= 5% contrast)
                Color::rgb(0.35, 0.5, 0.66), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
                Color::rgb(0.8, 0.844, 1.0), // atmospheric inscattering color (light gained due to scattering from the sun)
            ),
        },
        Fxaa::default(),
        BloomSettings {
            intensity: 0.3,
            composite_mode: BloomCompositeMode::Additive,
            prefilter_settings: BloomPrefilterSettings {
                threshold: 2.0,
                threshold_softness: 0.5,
            },
            ..Default::default()
        },
        PanOrbitCameraControls {
            radius,
            ..Default::default()
        },
        // AtmosphereCamera::default(),
        MainCamera,
    ));
}

#[allow(dead_code)]
fn move_particles_with_camera(
    mut particle_query: Query<(&mut EffectSpawner, &mut Transform), Without<Projection>>,
    camera_query: Query<&Transform, (With<Projection>, With<MainCamera>)>,
) {
    // Note: On first frame where the effect spawns, EffectSpawner is spawned during
    // CoreSet::PostUpdate, so will not be available yet. Ignore for a frame if
    // so.
    let Ok(mut effect) = particle_query.get_single_mut() else { return; };

    let Ok(transform) = camera_query.get_single() else { return; };

    if effect.1.translation == transform.translation {
        return;
    }

    effect.1.translation = transform.translation;
    effect.0.reset();
}

#[allow(dead_code)]
fn toggle_fog_system(keycode: Res<Input<KeyCode>>, mut fog: Query<&mut FogSettings>) {
    let mut fog_settings = fog.single_mut();

    if keycode.just_pressed(KeyCode::Space) {
        let a = fog_settings.color.a();
        fog_settings.color.set_a(1.0 - a);
    }

    if keycode.just_pressed(KeyCode::S) {
        let a = fog_settings.directional_light_color.a();
        fog_settings.directional_light_color.set_a(0.5 - a);
    }
}

#[allow(dead_code)]
fn update_bloom_settings(
    mut camera: Query<(Entity, Option<&mut BloomSettings>), With<Camera>>,
    mut text: Query<&mut Text>,
    mut commands: Commands,
    keycode: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let bloom_settings = camera.single_mut();
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;

    match bloom_settings {
        (entity, Some(mut bloom_settings)) => {
            *text = "BloomSettings (Toggle: Space)\n".to_string();
            text.push_str(&format!("(Q/A) Intensity: {}\n", bloom_settings.intensity));
            text.push_str(&format!(
                "(W/S) Low-frequency boost: {}\n",
                bloom_settings.low_frequency_boost
            ));
            text.push_str(&format!(
                "(E/D) Low-frequency boost curvature: {}\n",
                bloom_settings.low_frequency_boost_curvature
            ));
            text.push_str(&format!(
                "(R/F) High-pass frequency: {}\n",
                bloom_settings.high_pass_frequency
            ));
            text.push_str(&format!(
                "(T/G) Mode: {}\n",
                match bloom_settings.composite_mode {
                    BloomCompositeMode::EnergyConserving => "Energy-conserving",
                    BloomCompositeMode::Additive => "Additive",
                }
            ));
            text.push_str(&format!(
                "(Y/H) Threshold: {}\n",
                bloom_settings.prefilter_settings.threshold
            ));
            text.push_str(&format!(
                "(U/J) Threshold softness: {}\n",
                bloom_settings.prefilter_settings.threshold_softness
            ));

            if keycode.just_pressed(KeyCode::Space) {
                commands.entity(entity).remove::<BloomSettings>();
            }

            let dt = time.delta_seconds();

            if keycode.pressed(KeyCode::A) {
                bloom_settings.intensity -= dt / 10.0;
            }
            if keycode.pressed(KeyCode::Q) {
                bloom_settings.intensity += dt / 10.0;
            }
            bloom_settings.intensity = bloom_settings.intensity.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::S) {
                bloom_settings.low_frequency_boost -= dt / 10.0;
            }
            if keycode.pressed(KeyCode::W) {
                bloom_settings.low_frequency_boost += dt / 10.0;
            }
            bloom_settings.low_frequency_boost = bloom_settings.low_frequency_boost.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::D) {
                bloom_settings.low_frequency_boost_curvature -= dt / 10.0;
            }
            if keycode.pressed(KeyCode::E) {
                bloom_settings.low_frequency_boost_curvature += dt / 10.0;
            }
            bloom_settings.low_frequency_boost_curvature =
                bloom_settings.low_frequency_boost_curvature.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::F) {
                bloom_settings.high_pass_frequency -= dt / 10.0;
            }
            if keycode.pressed(KeyCode::R) {
                bloom_settings.high_pass_frequency += dt / 10.0;
            }
            bloom_settings.high_pass_frequency = bloom_settings.high_pass_frequency.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::G) {
                bloom_settings.composite_mode = BloomCompositeMode::Additive;
            }
            if keycode.pressed(KeyCode::T) {
                bloom_settings.composite_mode = BloomCompositeMode::EnergyConserving;
            }

            if keycode.pressed(KeyCode::H) {
                bloom_settings.prefilter_settings.threshold -= dt;
            }
            if keycode.pressed(KeyCode::Y) {
                bloom_settings.prefilter_settings.threshold += dt;
            }
            bloom_settings.prefilter_settings.threshold =
                bloom_settings.prefilter_settings.threshold.max(0.0);

            if keycode.pressed(KeyCode::J) {
                bloom_settings.prefilter_settings.threshold_softness -= dt / 10.0;
            }
            if keycode.pressed(KeyCode::U) {
                bloom_settings.prefilter_settings.threshold_softness += dt / 10.0;
            }
            bloom_settings.prefilter_settings.threshold_softness = bloom_settings
                .prefilter_settings
                .threshold_softness
                .clamp(0.0, 1.0);
        }

        (entity, None) => {
            *text = "Bloom: Off (Toggle: Space)".to_string();

            if keycode.just_pressed(KeyCode::Space) {
                commands.entity(entity).insert(BloomSettings::default());
            }
        }
    }
}

#[allow(dead_code)]
fn setup_instructions(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((TextBundle::from_section(
        "Press Spacebar to Toggle Atmospheric Fog.\nPress S to Toggle Directional Light Fog Influence.",
        TextStyle {
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            font_size: 15.0,
            color: Color::WHITE,
        },
    )
    .with_style(Style {
        position_type: PositionType::Absolute,
        position: UiRect {
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        ..default()
    }),));
}
