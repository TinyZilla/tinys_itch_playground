//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::prelude::*;
use bevy::dev_tools::states::log_transitions;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::core::states;

pub(super) fn plugin(app: &mut App) {
    // Print state transitions in dev builds
    app.add_systems(Update, log_transitions::<states::GameState>);
    // app.add_plugins(WorldInspectorPlugin::new());
}
