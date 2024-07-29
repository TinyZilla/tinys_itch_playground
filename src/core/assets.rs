mod audio_sfx;
mod audio_soundtrack;
mod mesh;

pub mod prelude {
    pub use crate::core::assets::audio_sfx::SfxKey;
    pub use crate::core::assets::audio_soundtrack::SoundtrackKey;
    pub use crate::core::assets::mesh::MeshKey;
}

use bevy::{
    prelude::*,
    utils::HashMap
};

pub trait AssetKey: Sized {
    type Asset: Asset;
}

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct HandleMap<K: AssetKey>(HashMap<K, Handle<K::Asset>>);

impl<K: AssetKey, T> From<T> for HandleMap<K>
where
    T: Into<HashMap<K, Handle<K::Asset>>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<K: AssetKey> HandleMap<K> {
    pub fn all_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}