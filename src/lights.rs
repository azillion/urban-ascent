use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};

use crate::{simulation::TimeConfig, AppState};

use crate::ui::{DAY, NIGHT};

#[derive(Component)]
pub struct Sun;

pub struct LightsPlugin;

impl Plugin for LightsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_sun);
        // .add_system(update_sun_transform.in_set(OnUpdate(AppState::InGame)));
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

pub fn update_sun_transform(
    time: Res<TimeConfig>,
    mut query: Query<(&Sun, &mut DirectionalLight, &mut Transform)>,
) {
    let day_percent = time.daylight_amount();
    // We'll rotate the sun 180 degrees over the course of a day,
    // from -90 to +90 degrees. Subtract 90 to start the sun just below the horizon.
    let rotation_angle = (day_percent * 180.0 - 90.0).to_radians();

    // Prepare the sun's colors for day and night
    let day_color = DAY;
    let night_color = NIGHT;

    // Iterate over all entities with the Sun component
    for (_, mut light, mut transform) in query.iter_mut() {
        transform.rotation = Quat::from_rotation_x(rotation_angle);

        // Adjust the light's color based on the time of day
        if day_percent < 0.5 {
            // Sunrise to midday, interpolate from night to day color
            let sunrise_percent = (day_percent * 2.0).clamp(0.0, 1.0);
            // light.color = night_color.lerp(day_color, sunrise_percent);
            light.color = color_lerp(night_color, day_color, sunrise_percent);
        } else {
            // Midday to sunset, interpolate from day to night color
            let sunset_percent = ((day_percent - 0.5) * 2.0).clamp(0.0, 1.0);
            // light.color = day_color.lerp(night_color, sunset_percent);
            light.color = color_lerp(day_color, night_color, sunset_percent);
        }
    }
}

fn color_lerp(a: Color, b: Color, t: f32) -> Color {
    let r = a.r() + (b.r() - a.r()) * t;
    let g = a.g() + (b.g() - a.g()) * t;
    let b = a.b() + (b.b() - a.b()) * t;
    Color::rgba(r, g, b, 1.0)
}
