//! The idea is that this is the entry point for the game's core logic.
//! The Components & State definition will live in the files in `core`.

use bevy::prelude::*;
use crate::core::state;

mod scenes;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<state::GameState>();
        app.enable_state_scoped_entities::<state::GameState>();

        // Get the Sub Systems up and running.
        app.add_plugins(scenes::plugin);
    }
}
