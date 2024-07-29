use bevy::prelude::{ 
    FromWorld,
    Reflect,
    AudioSource,
    AssetServer,
    World
};
use super::{AssetKey, HandleMap};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SoundtrackKey {
    Gameplay,
}

impl AssetKey for SoundtrackKey {
    type Asset = AudioSource;
}

impl FromWorld for HandleMap<SoundtrackKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                SoundtrackKey::Gameplay,
                asset_server.load("audio/soundtracks/zzz pull theme.ogg"),
            ),
        ]
        .into()
    }
}
