// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::{
    core_pipeline::prepass::DepthPrepass, pbr::{MaterialPipeline, MaterialPipelineKey, NotShadowCaster}, prelude::*, render::{
        mesh::MeshVertexBufferLayoutRef,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    }
};

fn main() -> AppExit {
    info!("Hello From Bevy");
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 500.,
        })
        // .init_asset::<CustomMaterial>()
        .add_plugins(MaterialPlugin::<CustomMaterial> {
            prepass_enabled: false,
            ..default()
        })
        .add_systems(Startup, spawn_stuff)
        .add_systems(Update, move_cube)
        .run()
}

#[derive(Component)]
struct ShieldObject;

fn move_cube(mut query: Query<&mut Transform, With<ShieldObject>>, key_press: Res<ButtonInput<KeyCode>>, time: Res<Time>) {

    const MOVE_SPEED: f32 = 5.;
    
    let mut move_factor = 0.;

    if key_press.pressed(KeyCode::KeyQ) {
        move_factor += MOVE_SPEED;
    }

    if key_press.pressed(KeyCode::KeyE) {
        move_factor -= MOVE_SPEED;
    }

    if move_factor != 0. {
        let mut shield_transform = query.single_mut();
        shield_transform.translation.y += move_factor * time.delta_seconds();
    }
}

fn spawn_stuff(mut commands: Commands, asset_server: Res<AssetServer>) {
    let custom_mat_handle = asset_server.add(CustomMaterial {});
    // Spawn the object
    commands.spawn((
        Name::new("Shaded Object"),
        MaterialMeshBundle {
            mesh: asset_server.add(Sphere::new(1.).into()),
            material: custom_mat_handle.clone(),
            transform: Transform::from_rotation(Quat::from_rotation_y(f32::to_radians(45.0))),
            ..default()
        },
        NotShadowCaster,
        ShieldObject
    ));

    // Spawn the Floor
    commands.spawn((
        Name::new("Floor"),
        MaterialMeshBundle {
            mesh: asset_server.add(Plane3d::new(Vec3::Y, Vec2::splat(5.0)).into()),
            material: asset_server.add(StandardMaterial {
                ..default()
            }),
            ..default()
        }
    ));

    // Spawn the Wall
    commands.spawn((
    Name::new("Wall"),
    MaterialMeshBundle {
        mesh: asset_server.add(Plane3d::new(Vec3::Y, Vec2::splat(5.0)).into()),
        material: asset_server.add(StandardMaterial {
            ..default()
        }),
        transform: Transform::from_translation(Vec3::new(0.75, 0., 0.)).with_rotation(Quat::from_rotation_z(f32::to_radians(90.))),
        ..default()
    }
    ));

    // Spawn the Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 3., 4.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        DepthPrepass,
    ));
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct CustomMaterial {}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}
