use bevy::prelude::*;

use bevy::input::common_conditions::input_just_pressed;

use crate::core::states::GameState;

use crate::game::asset_loader;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Loading), (setup_loading_screen, asset_loader::load_assets));
    app.add_systems(
        FixedUpdate,
        continue_to_game.run_if(in_state(GameState::Loading).and_then(asset_loader::all_assets_loaded).and_then(input_just_pressed(KeyCode::Enter).or_else(input_just_pressed(KeyCode::Space)))),
    );
}

fn continue_to_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::InGame);
}

fn setup_loading_screen(mut commands: Commands) {

    // Setup the UI
    commands.spawn((
        Name::new("Header Text"),
        TextBundle::from_section(
            "Loading",
            TextStyle {
                font_size: 40.0,
                color: Color::srgb(0.867, 0.827, 0.412),
                ..default()
            },
        ),
        StateScoped(GameState::Loading)
    ));

    // Spawn a Camera to show the UI Text.
    commands.spawn((
        Name::new("Camera"),
        Camera2dBundle::default(),
        IsDefaultUiCamera,
        StateScoped(GameState::Loading)
    ));
}