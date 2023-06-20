use bevy::{pbr::NotShadowCaster, prelude::*};

const TERRAIN_SIZE: f32 = 10000.0;
const SKY_COLOR_HEX: &str = "888888";

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_terrain);
    }
}

fn setup_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(TERRAIN_SIZE).into()),
        material: materials.add(Color::DARK_GREEN.into()),
        ..default()
    });

    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                ..Default::default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::TOMATO.into(),
                perceptual_roughness: 0.7,
                metallic: 0.9,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.1, 0.0),
            ..default()
        },
        // PickableBundle::default(),
        // RaycastPickTarget::default(),
    ));

    // Sky
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::default())),
            material: materials.add(StandardMaterial {
                base_color: Color::hex(SKY_COLOR_HEX).unwrap(),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(2000.0)),
            ..default()
        },
        NotShadowCaster,
    ));
}
