use crate::core::assets::{
    HandleMap,
    prelude::*
};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<HandleMap<SfxKey>>();
    app.register_type::<HandleMap<SoundtrackKey>>();
    app.register_type::<HandleMap<MeshKey>>();
}

pub fn load_assets(mut commands: Commands) {
    commands.init_resource::<HandleMap<SfxKey>>();
    commands.init_resource::<HandleMap<SoundtrackKey>>();
    commands.init_resource::<HandleMap<MeshKey>>();
}

pub fn all_assets_loaded(
    asset_server: Res<AssetServer>,
    sfx_handles: Res<HandleMap<SfxKey>>,
    soundtrack_handles: Res<HandleMap<SoundtrackKey>>,
    mesh_handles: Res<HandleMap<MeshKey>>
) -> bool {
    sfx_handles.all_loaded(&asset_server)
        && soundtrack_handles.all_loaded(&asset_server)
        && mesh_handles.all_loaded(&asset_server)
}