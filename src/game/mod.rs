//! The idea is that this is the entry point for the game's core logic.
//! The Components & State definition will live in the files in `core`.

use bevy::prelude::*;
use crate::core::states;

mod scenes;
mod asset_loader;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<states::GameState>();
        app.enable_state_scoped_entities::<states::GameState>();

        // Get Resource Initialization online.
        app.add_plugins(asset_loader::plugin);

        // Get the Sub Systems up and running.
        app.add_plugins(scenes::plugin);
    }
}
