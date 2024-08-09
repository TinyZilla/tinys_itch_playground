mod debug;
mod movement;
mod spaceship;
mod camera;

use bevy::prelude::*;
use avian3d::prelude::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        // Bevy Built-in
        .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.0,
        })
        .add_plugins(DefaultPlugins)

        // External Crates
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec3::ZERO))
        .add_plugins(PhysicsDebugPlugin::default())

        // User Configured Plugins
        .add_plugins(CameraPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(MovementPlugin)
        // .add_plugins(DebugPlugin)
        .run();
}
