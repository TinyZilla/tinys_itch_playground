// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;

fn main() -> AppExit {
    info!("Hello From Bevy");
    App::new()
        .run()
}
