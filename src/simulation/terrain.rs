use bevy::{pbr::NotShadowCaster, prelude::*};
// use bevy_rapier3d::prelude::*;
// use noise::{NoiseFn, Perlin};

pub const TERRAIN_SIZE: f32 = 100000000.0;
pub const TERRAIN_LENGTH: f32 = 10000.0;
pub const TERRAIN_WIDTH: f32 = 10000.0;
pub const SKY_HEIGHT: f32 = 1000.0;
pub const TERRAIN_HEIGHT: f32 = -0.1;
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
        .spawn((
            PbrBundle {
                mesh: meshes.add(shape::Plane::from_size(TERRAIN_SIZE).into()),
                material: materials.add(Color::rgb(0.15, 0.6, 0.46).into()),
                transform: Transform::from_xyz(0.0, TERRAIN_HEIGHT, 0.0),
                ..default()
            },
            // Collider::cuboid(TERRAIN_LENGTH, TERRAIN_HEIGHT, TERRAIN_WIDTH),
        ))
        .insert(Terrain);

    // cube
    // let rainbow_colors: [Color; 7] = [
    //     Color::rgb(1.0, 0.0, 0.0),
    //     Color::rgb(1.0, 0.5, 0.0),
    //     Color::rgb(1.0, 1.0, 0.0),
    //     Color::rgb(0.0, 1.0, 0.0),
    //     Color::rgb(0.0, 0.0, 1.0),
    //     Color::rgb(0.29, 0.0, 0.51),
    //     Color::rgb(0.58, 0.0, 0.83),
    // ];

    // let perlin = Perlin::new(42);

    // let building_size = 4.0;

    // let mut mat = StandardMaterial {
    //     base_color: Color::rgb(0.0, 0.0, 0.0).into(),
    //     perceptual_roughness: 0.7,
    //     metallic: 0.9,
    //     ..default()
    // };

    // for x in (0..100).map(|i| i as f32 * building_size) {
    //     for z in (0..100).map(|i| i as f32 * building_size) {
    //         let val = perlin
    //             .get([x as f64 * 0.1, z as f64 * 0.1, rand::random()])
    //             .abs();
    //         let max_y = (val * 50.0 + 5.0) as f32;

    //         let color: Color = rainbow_colors[(val * 6.0).floor() as usize];

    //         mat.base_color = color.into();

    //         // create buildings
    //         commands.spawn((PbrBundle {
    //             mesh: meshes.add(Mesh::from(shape::Box {
    //                 min_x: -building_size / 2.0,
    //                 max_x: building_size / 2.0,
    //                 min_z: -building_size / 2.0,
    //                 max_z: building_size / 2.0,
    //                 min_y: 0.1,
    //                 max_y: max_y,
    //                 ..Default::default()
    //             })),
    //             material: materials.add(mat.clone()),
    //             transform: Transform::from_xyz(x - 50., 0.1, z - 50.),
    //             ..default()
    //         },));
    //     }
    // }

    // Sky
    let sky_mesh = Mesh::from(shape::Box::new(TERRAIN_LENGTH, SKY_HEIGHT, TERRAIN_WIDTH));
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
