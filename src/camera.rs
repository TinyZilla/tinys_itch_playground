use bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 20.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0., CAMERA_DISTANCE, 0.)).with_rotation(Quat::from_rotation_x(f32::to_radians(-90.))),
        ..default()
    });
}