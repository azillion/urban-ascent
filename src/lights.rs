use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};
use bevy_atmosphere::prelude::*;

use crate::{simulation::TimeConfig, AppState};

use crate::ui::{DAY, NIGHT};

#[derive(Component)]
pub struct Sun;

// Timer for updating the daylight cycle (updating the atmosphere every frame is slow, so it's better to do incremental changes)
#[derive(Resource)]
struct CycleTimer(Timer);

pub struct LightsPlugin;

impl Plugin for LightsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AtmosphereModel::default()) // Default Atmosphere material, we can edit it to simulate another planet
            // .insert_resource(CycleTimer(Timer::new(
            //     bevy::utils::Duration::from_millis(50), // Update our atmosphere every 50ms (in a real game, this would be much slower, but for the sake of an example we use a faster update)
            //     TimerMode::Repeating,
            // )))
            .insert_resource(AtmosphereModel::new(Gradient::default()))
            .add_plugin(AtmospherePlugin)
            .add_startup_system(setup_sun)
            .add_system(change_gradient);
        // .add_system(daylight_cycle);
        // .add_system(update_sun_transform.in_set(OnUpdate(AppState::InGame)));
    }
}

fn change_gradient(mut commands: Commands, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Key1) {
        info!("Changed to Atmosphere Preset 1 (Default Gradient)");
        commands.insert_resource(AtmosphereModel::new(Gradient::default()));
    } else if keys.just_pressed(KeyCode::Key2) {
        info!("Changed to Atmosphere Preset 2 (Cotton Candy)");
        commands.insert_resource(AtmosphereModel::new(Gradient {
            ground: Color::rgb(1.0, 0.5, 0.75),
            horizon: Color::WHITE,
            sky: Color::rgb(0.5, 0.75, 1.0),
        }));
    } else if keys.just_pressed(KeyCode::Key3) {
        info!("Changed to Atmosphere Preset 3 (80's Sunset)");
        commands.insert_resource(AtmosphereModel::new(Gradient {
            sky: Color::PURPLE,
            horizon: Color::PINK,
            ground: Color::ORANGE,
        }));
    } else if keys.just_pressed(KeyCode::Key4) {
        info!("Changed to Atmosphere Preset 4 (Winter)");
        commands.insert_resource(AtmosphereModel::new(Gradient {
            ground: Color::rgb(0.0, 0.1, 0.2),
            horizon: Color::rgb(0.3, 0.4, 0.5),
            sky: Color::rgb(0.7, 0.8, 0.9),
        }));
    } else if keys.just_pressed(KeyCode::Key5) {
        info!("Changed to Atmosphere Preset 5 (Nether)");
        commands.insert_resource(AtmosphereModel::new(Gradient {
            ground: Color::BLACK,
            horizon: Color::rgb(0.2, 0.0, 0.0),
            sky: Color::rgb(0.5, 0.1, 0.0),
        }));
    } else if keys.just_pressed(KeyCode::Key6) {
        info!("Changed to Atmosphere Preset 6 (Golden)");
        commands.insert_resource(AtmosphereModel::new(Gradient {
            ground: Color::ORANGE_RED,
            horizon: Color::ORANGE,
            sky: Color::GOLD,
        }));
    } else if keys.just_pressed(KeyCode::Key7) {
        info!("Changed to Atmosphere Preset 7 (Noir)");
        commands.insert_resource(AtmosphereModel::new(Gradient {
            ground: Color::BLACK,
            horizon: Color::BLACK,
            sky: Color::WHITE,
        }));
    } else if keys.just_pressed(KeyCode::Key8) {
        info!("Changed to Atmosphere Preset 8 (Midnight)");
        commands.insert_resource(AtmosphereModel::new(Gradient {
            ground: Color::BLACK,
            horizon: Color::BLACK,
            sky: Color::MIDNIGHT_BLUE,
        }));
    } else if keys.just_pressed(KeyCode::Key9) {
        info!("Changed to Atmosphere Preset 9 (Greenery)");
        commands.insert_resource(AtmosphereModel::new(Gradient {
            ground: Color::rgb(0.1, 0.2, 0.0),
            horizon: Color::rgb(0.3, 0.4, 0.1),
            sky: Color::rgb(0.6, 0.8, 0.2),
        }));
    } else if keys.just_pressed(KeyCode::Key0) {
        info!("Reset Atmosphere to Default");
        commands.remove_resource::<AtmosphereModel>();
    }
}

fn setup_sun(mut commands: Commands) {
    // Configure a properly scaled cascade shadow map for this scene (defaults are too large, mesh units are in km)
    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 0.3,
        maximum_distance: 3.0,
        ..default()
    }
    .build();

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                color: DAY,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 500.0, 0.0)
                .looking_at(Vec3::new(-0.15, -0.05, 0.25), Vec3::Y),
            cascade_shadow_config,
            ..default()
        },
        Sun,
    ));
}

fn daylight_cycle(
    mut atmosphere: AtmosphereMut<Nishita>,
    mut query: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
    mut timer: ResMut<CycleTimer>,
    time: Res<Time>,
    day_time: Res<TimeConfig>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        let t = time.elapsed_seconds_wrapped() as f32 / 2.0;
        atmosphere.sun_position = Vec3::new(0., t.sin(), t.cos());

        if let Some((mut light_trans, mut directional)) = query.single_mut().into() {
            light_trans.rotation = Quat::from_rotation_x(-t);
            directional.illuminance = t.sin().max(0.0).powf(2.0) * 100000.0;
        }
    }
}

// pub fn update_sun_transform(
//     time: Res<TimeConfig>,
//     mut query: Query<(&Sun, &mut DirectionalLight, &mut Transform)>,
// ) {
//     let day_percent = time.daylight_amount();
//     // We'll rotate the sun 180 degrees over the course of a day,
//     // from -90 to +90 degrees. Subtract 90 to start the sun just below the horizon.
//     let rotation_angle = (day_percent * 180.0 - 90.0).to_radians();

//     // Prepare the sun's colors for day and night
//     let day_color = DAY;
//     let night_color = NIGHT;

//     // Iterate over all entities with the Sun component
//     for (_, mut light, mut transform) in query.iter_mut() {
//         transform.rotation = Quat::from_rotation_x(rotation_angle);

//         // Adjust the light's color based on the time of day
//         if day_percent < 0.5 {
//             // Sunrise to midday, interpolate from night to day color
//             let sunrise_percent = (day_percent * 2.0).clamp(0.0, 1.0);
//             // light.color = night_color.lerp(day_color, sunrise_percent);
//             light.color = color_lerp(night_color, day_color, sunrise_percent);
//         } else {
//             // Midday to sunset, interpolate from day to night color
//             let sunset_percent = ((day_percent - 0.5) * 2.0).clamp(0.0, 1.0);
//             // light.color = day_color.lerp(night_color, sunset_percent);
//             light.color = color_lerp(day_color, night_color, sunset_percent);
//         }
//     }
// }

fn color_lerp(a: Color, b: Color, t: f32) -> Color {
    let r = a.r() + (b.r() - a.r()) * t;
    let g = a.g() + (b.g() - a.g()) * t;
    let b = a.b() + (b.b() - a.b()) * t;
    Color::rgba(r, g, b, 1.0)
}
