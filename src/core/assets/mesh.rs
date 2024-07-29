use bevy::prelude::{ 
    FromWorld,
    Reflect,
    AssetServer,
    World,
    Mesh,
    GltfAssetLabel
};
use super::{AssetKey, HandleMap};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum MeshKey {
    Torus,
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
        )]
        .into()
    }
}
