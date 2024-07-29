
use bevy::prelude::*;

use crate::core::states::GameState;
use crate::core::assets::HandleMap;
use crate::core::assets::prelude::*;

use bevy_panorbit_camera::PanOrbitCamera;


pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), enter_in_game);
}

fn enter_in_game(mut commands: Commands, mesh_handles: Res<HandleMap<MeshKey>>, mut materials:ResMut<Assets<StandardMaterial>>) {

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
        StateScoped(GameState::InGame)
    ));

    // Spawn a Floor
    commands.spawn((
        Name::new("Floor"),
        PbrBundle {
            mesh: mesh_handles[&MeshKey::Plane].clone_weak(),
            material: material_handle.clone(),
            transform: Transform::from_scale(Vec3::splat(10.0)),
            ..default()
        },
        StateScoped(GameState::InGame)
    ));

    // Spawn a light
    commands.spawn((
        Name::new("Point Light"),
        PointLightBundle{
            transform: Transform::from_xyz(4.0, 5.0, 4.0),
            ..default()
        },
        StateScoped(GameState::InGame)
    ));

    // Setup the Camera
    commands.spawn((
        Name::new("Camera"),
        Camera3dBundle{
            transform: Transform::from_xyz(0.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        IsDefaultUiCamera,
        PanOrbitCamera::default(),
        StateScoped(GameState::InGame)
    ));
    
}