use bevy::{pbr::NotShadowCaster, prelude::*};

const TERRAIN_SIZE: f32 = 10000.0;
const SKY_COLOR_HEX: &str = "87CEEB";

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
        material: materials.add(Color::rgb(0.15, 0.6, 0.46).into()),
        ..default()
    });

    // cube
    for x in -50..50 {
        for z in -50..50 {
            if x % 2 == 0 && z % 2 == 0 {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Box {
                            ..Default::default()
                        })),
                        material: materials.add(StandardMaterial {
                            base_color: Color::DARK_GRAY.into(),
                            perceptual_roughness: 0.7,
                            metallic: 0.9,
                            ..default()
                        }),
                        transform: Transform::from_xyz(x as f32 * 2.0, 0.1, z as f32 * 2.0),
                        ..default()
                    },
                    // PickableBundle::default(),
                    // RaycastPickTarget::default(),
                ));
                continue;
            }
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
                    transform: Transform::from_xyz(x as f32 * 2.0, 0.1, z as f32 * 2.0),
                    ..default()
                },
                // PickableBundle::default(),
                // RaycastPickTarget::default(),
            ));
        }
    }

    // Sky
    let sky_mesh = Mesh::from(shape::Box::new(10000.0, 1000.0, 10000.0));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(sky_mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::hex(SKY_COLOR_HEX).unwrap(),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            ..default()
        },
        NotShadowCaster,
    ));
}
