use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension, OpaqueRendererMethod},
    prelude::*,
    render::render_resource::*,
};


const STARTING_TRANSLATION: Vec3 = Vec3::ZERO;

#[derive(Component)]
pub struct ShipSheild;

pub struct ShipSheildPlugin;

impl Plugin for ShipSheildPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ship_shield);
        app.add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, MyExtension>,
        >::default());
        // app.add_systems(Update, (speen_spaceship_on_button_press, move_spaceship_on_button_press));
    }
}

fn spawn_ship_shield(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sphere_handle: Handle<Mesh> = asset_server.add(Sphere::new(1.).into());
    let material_handle:Handle<ExtendedMaterial<StandardMaterial, MyExtension>> = asset_server.add(
        ExtendedMaterial {
            base: StandardMaterial {
                base_color: Color::Srgba(Srgba::rgb_u8(0, 255, 0)),
                opaque_render_method: OpaqueRendererMethod::Auto,
                ..default()
            },
            extension: MyExtension { quantize_steps: 3 },
        });
    commands.spawn(sphere(sphere_handle, material_handle));

    // spawn a pointlight near the sphere
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            ..default()
        },
        transform: Transform::from_translation(Vec3::Z * 2.),
        ..default()
    });
}

fn sphere(sphere_handle: Handle<Mesh>, material_handle: Handle<ExtendedMaterial<StandardMaterial, MyExtension>>) -> (Name, ShipSheild, MaterialMeshBundle<ExtendedMaterial<StandardMaterial, MyExtension>>) {
    (
        Name::new("Ship Shield"),
        ShipSheild,
        MaterialMeshBundle {
            mesh: sphere_handle,
            material: material_handle,
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        }
    )
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
struct MyExtension {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(100)]
    quantize_steps: u32,
}

impl MaterialExtension for MyExtension {
    fn fragment_shader() -> ShaderRef {
        "shaders/extended_material.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/extended_material.wgsl".into()
    }
}