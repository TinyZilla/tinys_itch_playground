
use bevy::prelude::*;

use crate::core::state::GameState;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), enter_in_game);
}

fn enter_in_game(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials:ResMut<Assets<StandardMaterial>>) {

    // Make a material for the Cube
    // You can also add assets directly to their Assets<T> storage:
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.7, 0.6),
        ..default()
    });
    
    // Load the Cube As a mesh.
    let cube_handle = meshes.add(Cuboid::default());

    // Spawn a cube
    commands.spawn((
        Name::new("Cube"),
        PbrBundle {
            mesh: cube_handle,
            material: material_handle.clone(),
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

    commands.spawn((
        Name::new("Camera"),
        Camera3dBundle{
            transform: Transform::from_xyz(0.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        IsDefaultUiCamera,
        StateScoped(GameState::InGame)
    ));
    
}