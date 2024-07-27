//! Spawn the Torus.

use bevy::prelude::*;

use crate::{
    game::assets::{HandleMap, MeshKey},
    screen::Screen,
    AppSet,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_world);
    app.register_type::<World>();
    app.add_systems(Update, rotate.in_set(AppSet::Update));
}

#[derive(Event, Debug)]
pub struct SpawnWorld;

#[derive(Component)]
struct Shape;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct World;

fn spawn_world(
    _trigger: Trigger<SpawnWorld>,
    mut commands: Commands,
    mesh_handles: Res<HandleMap<MeshKey>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // You can also add assets directly to their Assets<T> storage:
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.7, 0.6),
        ..default()
    });

    commands.spawn((
        Name::new("torus"),
        Shape,
        PbrBundle {
            mesh: mesh_handles[&MeshKey::Torus].clone_weak(),
            material: material_handle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        StateScoped(Screen::Playing),
    ));

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 5.0, 4.0),
        ..default()
    });

}

fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}
