use bevy::{pbr::NotShadowCaster, prelude::*};
use noise::{NoiseFn, Perlin};

pub const TERRAIN_SIZE: f32 = 100000000.0;
pub const TERRAIN_LENGTH: f32 = 10000.0;
pub const TERRAIN_WIDTH: f32 = 10000.0;
pub const TERRAIN_HEIGHT: f32 = 1000.0;
const SKY_COLOR_HEX: &str = "87CEEB";

// 10000^2 = 100000000

#[derive(Component)]
pub struct Terrain;

#[derive(Component)]
pub struct Sky;

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
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(TERRAIN_SIZE).into()),
            material: materials.add(Color::rgb(0.15, 0.6, 0.46).into()),
            ..default()
        })
        .insert(Terrain);

    // cube
    let rainbow_colors: [Color; 7] = [
        Color::rgb(1.0, 0.0, 0.0),
        Color::rgb(1.0, 0.5, 0.0),
        Color::rgb(1.0, 1.0, 0.0),
        Color::rgb(0.0, 1.0, 0.0),
        Color::rgb(0.0, 0.0, 1.0),
        Color::rgb(0.29, 0.0, 0.51),
        Color::rgb(0.58, 0.0, 0.83),
    ];

    let perlin = Perlin::new(42);

    let building_size = 2.0;

    for x in (0..100).map(|i| i as f32 * building_size) {
        for z in (0..100).map(|i| i as f32 * building_size) {
            let val = perlin
                .get([x as f64 * 0.1, z as f64 * 0.1, rand::random()])
                .abs();
            let max_y = (val * 50.0 + 5.0) as f32;

            let color: Color = rainbow_colors[(val * 6.0).floor() as usize];

            // create roads
            if ((x / building_size) as i32) % 5 == 0 || ((z / building_size) as i32) % 5 == 0 {
                commands.spawn((PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box {
                        min_y: 0.0,
                        max_y: 0.1,
                        ..Default::default()
                    })),
                    material: materials.add(StandardMaterial {
                        base_color: Color::GRAY.into(),
                        perceptual_roughness: 0.7,
                        metallic: 0.0,
                        ..default()
                    }),
                    transform: Transform::from_xyz(x, 0.1, z),
                    ..default()
                },));
                continue;
            }

            // create buildings
            commands.spawn((PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box {
                    min_x: -building_size / 2.0,
                    max_x: building_size / 2.0,
                    min_z: -building_size / 2.0,
                    max_z: building_size / 2.0,
                    min_y: 0.0,
                    max_y: max_y,
                    ..Default::default()
                })),
                material: materials.add(StandardMaterial {
                    base_color: color.into(),
                    perceptual_roughness: 0.7,
                    metallic: 0.9,
                    ..default()
                }),
                transform: Transform::from_xyz(x, max_y / 2.0, z),
                ..default()
            },));
        }
    }

    // Sky
    let sky_mesh = Mesh::from(shape::Box::new(
        TERRAIN_LENGTH,
        TERRAIN_HEIGHT,
        TERRAIN_WIDTH,
    ));
    commands
        .spawn((
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
        ))
        .insert(Sky);
}
