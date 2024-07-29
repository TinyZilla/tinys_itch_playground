use bevy::audio::PlaybackMode;
use bevy::prelude::*;

use std::f32::consts::PI;

use crate::core::assets::prelude::*;
use crate::core::assets::HandleMap;
use crate::core::states::GameState;

use bevy_panorbit_camera::PanOrbitCamera;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), enter_in_game);
}

fn enter_in_game(
    mut commands: Commands,
    mesh_handles: Res<HandleMap<MeshKey>>,
    soundtrack_handles: Res<HandleMap<SoundtrackKey>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Make a material for the Cube
    // You can also add assets directly to their Assets<T> storage:
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.7, 0.6),
        ..default()
    });

    // Spawn a Mesh
    commands.spawn((
        Name::new("torus"),
        PbrBundle {
            mesh: mesh_handles[&MeshKey::Torus].clone_weak(),
            material: material_handle.clone(),
            transform: Transform::from_translation(Vec3::Y * 1.25),
            ..default()
        },
        StateScoped(GameState::InGame),
    ));

    const MESH_SPACING: f32 = 3.5;
    // Spawn a Matrix of Mesh
    for i in 0..10 {
        commands.spawn((
            Name::new("torus"),
            PbrBundle {
                mesh: mesh_handles[&MeshKey::Torus].clone_weak(),
                material: material_handle.clone(),
                transform: Transform::from_xyz(
                    MESH_SPACING * i as f32,
                    1.25,
                    MESH_SPACING
                )
                .with_rotation(Quat::from_rotation_y(-PI / 4. * i as f32)),
                ..default()
            },
            StateScoped(GameState::InGame),
        ));
    }

    // Spawn a Floor
    commands.spawn((
        Name::new("Floor"),
        PbrBundle {
            mesh: mesh_handles[&MeshKey::Plane].clone_weak(),
            material: material_handle.clone(),
            transform: Transform::IDENTITY,
            ..default()
        },
        StateScoped(GameState::InGame),
    ));

    // Spawn a light
    commands.spawn((
        Name::new("Point Light"),
        PointLightBundle {
            point_light: PointLight {
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 5.0, 4.0),
            ..default()
        },
        StateScoped(GameState::InGame),
    ));

    // Setup the Camera
    commands.spawn((
        Name::new("Camera"),
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        IsDefaultUiCamera,
        PanOrbitCamera::default(),
        StateScoped(GameState::InGame),
    ));

    // Play the Soundtrack
    commands.spawn((
        Name::new("BGM Player"),
        AudioSourceBundle {
            source: soundtrack_handles[&SoundtrackKey::Gameplay].clone_weak(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                ..default()
            },
            ..default()
        },
        StateScoped(GameState::InGame),
    ));
}
