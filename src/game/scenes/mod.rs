//! The Scenes for the game.

mod in_game;
mod loading;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        in_game::plugin,
        loading::plugin
    ));
}