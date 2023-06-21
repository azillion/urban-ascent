use bevy::prelude::*;
use bevy_hanabi::prelude::*;

#[derive(Component)]
pub struct ParticleSystem;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HanabiPlugin)
            .add_startup_system(setup_gradient_particles);
        // .add_startup_system(setup_particles);
    }
}

fn setup_gradient_particles(mut effects: ResMut<Assets<EffectAsset>>, mut commands: Commands) {
    let mut color_gradient1 = Gradient::new();
    color_gradient1.add_key(0.0, Vec4::new(1.0, 1.0, 1.0, 1.0)); // white
    color_gradient1.add_key(0.5, Vec4::new(1.0, 0.5, 0.5, 1.0)); // soft red
    color_gradient1.add_key(1.0, Vec4::new(1.0, 0.5, 0.5, 1.0)); // soft red

    let mut size_gradient1 = Gradient::new();
    size_gradient1.add_key(0.0, Vec2::new(0.2, 0.2));
    size_gradient1.add_key(1.0, Vec2::splat(0.0));

    let effect1 = effects.add(
        EffectAsset {
            name: "cozy_particles".to_string(),
            capacity: 32768,
            spawner: Spawner::rate(120.0.into()),
            ..Default::default()
        }
        .init(InitPositionCone3dModifier {
            base_radius: 100.,
            top_radius: 30.,
            height: 10.,
            dimension: ShapeDimension::Volume,
            ..Default::default()
        })
        // Make spawned particles move away from the emitter origin
        .init(InitVelocitySphereModifier {
            center: Vec3::ZERO,
            speed: Value::Single(-1.0),
        })
        .init(InitLifetimeModifier {
            lifetime: Value::Uniform((10.0, 12.0)),
        })
        .update(LinearDragModifier { drag: 2. })
        .update(AccelModifier::constant(Vec3::new(0.01, -0.1, 0.01)))
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient1,
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient1,
        })
        .render(OrientAlongVelocityModifier),
    );

    commands
        .spawn((
            Name::new("cozy_particles"),
            ParticleEffectBundle {
                effect: ParticleEffect::new(effect1),
                transform: Transform::from_xyz(-5.0, -10., 5.0),
                ..Default::default()
            },
        ))
        .insert(ParticleSystem);
}

#[allow(dead_code)]
fn setup_billboard_particles(
    mut effects: ResMut<Assets<EffectAsset>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("cloud.png");

    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::ONE);
    gradient.add_key(0.5, Vec4::ONE);
    gradient.add_key(1.0, Vec4::new(1.0, 1.0, 1.0, 0.0));

    let effect = effects.add(
        EffectAsset {
            name: "Gradient".to_string(),
            // TODO: Figure out why no particle spawns if this is 1
            capacity: 32768,
            spawner: Spawner::rate(64.0.into()),
            ..Default::default()
        }
        .init(InitPositionCircleModifier {
            center: Vec3::Y * 0.1,
            axis: Vec3::Y,
            radius: 1.0,
            dimension: ShapeDimension::Volume,
        })
        .init(InitVelocityCircleModifier {
            center: Vec3::ZERO,
            axis: Vec3::Y,
            speed: Value::Uniform((0.7, 0.5)),
        })
        .init(InitLifetimeModifier {
            lifetime: 5_f32.into(),
        })
        .render(ParticleTextureModifier {
            texture: texture_handle,
        })
        .render(BillboardModifier {})
        .render(ColorOverLifetimeModifier { gradient })
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::constant([0.2; 2].into()),
        }),
    );

    // The ground
    // commands
    //     .spawn(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Plane {
    //             size: 4.0,
    //             ..default()
    //         })),
    //         material: materials.add(Color::BLUE.into()),
    //         transform: Transform::from_xyz(0.0, -0.5, 0.0),
    //         ..Default::default()
    //     })
    //     .insert(Name::new("ground"));

    commands
        .spawn(ParticleEffectBundle::new(effect))
        .insert(Name::new("effect"));
}
