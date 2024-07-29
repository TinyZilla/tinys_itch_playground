use bevy::prelude::*;
use super::{AssetKey, HandleMap};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum MeshKey {
    Torus,
    Plane,
}

impl AssetKey for MeshKey {
    type Asset = Mesh;
}

impl FromWorld for HandleMap<MeshKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [(
            MeshKey::Torus,
            asset_server.load(
                GltfAssetLabel::Primitive {
                    mesh: 0,
                    primitive: 0,
                }
                .from_asset("mesh/torus.gltf"),
            ),
        ),
        (
            MeshKey::Plane,
            asset_server.add(Plane3d::new(Vec3::new(0., 1., 0.), Vec2::new(60., 60.)).into())
        )]
        .into()
    }
}
