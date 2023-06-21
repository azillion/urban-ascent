use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};

#[derive(Component)]
pub struct Sun;

pub struct LightsPlugin;

impl Plugin for LightsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_sun);
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
                color: Color::rgb(0.98, 0.95, 0.82),
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 100.0, 0.0)
                .looking_at(Vec3::new(-0.15, -0.05, 0.25), Vec3::Y),
            cascade_shadow_config,
            ..default()
        },
        Sun,
    ));
}
