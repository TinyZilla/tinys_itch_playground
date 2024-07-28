//! The Scenes for the game.

mod in_game;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(in_game::plugin);
}