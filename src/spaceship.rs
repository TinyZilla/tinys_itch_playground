use avian3d::prelude::*;
use bevy::{math::VectorSpace, prelude::*};

const INERTIA_SCALAR: f32 = 50.0;
const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., 0.);

#[derive(Component)]
pub struct Spaceship;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship);
        app.add_systems(Update, (speen_spaceship_on_button_press, move_spaceship_on_button_press));
    }
}

fn spawn_spaceship(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("Spaceship"),
        MassPropertiesBundle {
            mass: Mass(0.05),
            inertia: Inertia(Mat3::IDENTITY.mul_scalar(INERTIA_SCALAR)),
            center_of_mass: CenterOfMass(Vec3::Y * -0.25),
            ..default()
        },
        RigidBody::Dynamic,
        LinearDamping(0.8),
        AngularDamping(0.8),
        SceneBundle {
            scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("spaceship.glb")),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        },
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh)
            .with_default_density(0.0),
        Spaceship
    ));
}

fn speen_spaceship_on_button_press(mut query: Query<&mut ExternalAngularImpulse, With<Spaceship>>, key_press: Res<ButtonInput<KeyCode>>) {
    if key_press.just_pressed(KeyCode::Space) {
        info!("Pressed Space Bar");
        let mut spaceship_ang_impulse: Mut<ExternalAngularImpulse> = query.single_mut();
        spaceship_ang_impulse.apply_impulse(Vec3::Y * 500.);
        // spaceship_transform.rotate_local_z(f32::to_radians(180.0) * time.delta_seconds());    
    }
}

fn move_spaceship_on_button_press(mut query: Query<(&mut ExternalImpulse, &GlobalTransform), With<Spaceship>>, key_press: Res<ButtonInput<KeyCode>>) {
    if key_press.just_pressed(KeyCode::KeyQ) {
        info!("Pressed Q key");
        let (mut external_force, global_transform) = query.single_mut();
        external_force.with_persistence(false);
        external_force.apply_impulse(-global_transform.forward().as_vec3());
    }
}