use bevy::prelude::*;
use bevy::ui::Val::*;

use crate::core::assets::{
    HandleMap,
    prelude::*
};

use crate::core::components::UiRoot;

use bevy::input::common_conditions::input_just_pressed;

use crate::core::states::GameState;

use crate::game::asset_loader;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Loading),
        (setup_loading_ui, start_loading_asset),
    );

    app.add_systems(FixedUpdate, poll_asset_loaded.run_if(in_state(GameState::Loading).and_then(any_with_component::<AssetLoadChecker>)));

    app.add_systems(
        FixedUpdate,
        continue_to_game.run_if(
            in_state(GameState::Loading)
                .and_then(asset_loader::all_assets_loaded)
                .and_then(
                    input_just_pressed(KeyCode::Enter).or_else(input_just_pressed(KeyCode::Space)),
                ),
        ),
    );
}

/********************************************************
 *              For Asset Load Checking..
 *  The general idea is that Spawn a Load Checker to run the polling system. Once it's loaded Trigger a system and unmount itself.
********************************************************/
#[derive(Component)]
struct AssetLoadChecker;

#[derive(Event)]
struct AssetLoaded;

fn start_loading_asset(mut commands: Commands) {
    commands.spawn(AssetLoadChecker);
    asset_loader::load_assets(commands);
}

fn poll_asset_loaded(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sfx_handles: Res<HandleMap<SfxKey>>,
    soundtrack_handles: Res<HandleMap<SoundtrackKey>>,
    mesh_handles: Res<HandleMap<MeshKey>>,
    loader_query: Query<Entity, With<AssetLoadChecker>>,
    ui_root_query: Query<Entity, With<UiRoot>>,
) {
    if !asset_loader::all_assets_loaded(asset_server, sfx_handles, soundtrack_handles, mesh_handles) {
        return;
    }

    let asset_load_checker = loader_query.single();
    commands.entity(asset_load_checker).despawn();

    commands.trigger_targets(AssetLoaded, ui_root_query.single());
}

fn continue_to_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::InGame);
}

fn on_asset_loaded(_trigger: Trigger<AssetLoaded>, mut commands: Commands, mut query: Query<Entity, With<UiRoot>>) {
    println!("loaded");
    // Spawn Continue Text to UI Root
    let ui_root = query.single_mut();

    commands.entity(ui_root).with_children(|parent| {
        parent.spawn((
            Name::new("Header Text"),
            TextBundle::from_section(
                "Press [Space] or [Enter] To Continue",
                TextStyle {
                    font_size: 40.0,
                    color: Color::srgb(0.867, 0.827, 0.412),
                    ..default()
                },
            ),
        ));
    });
}

fn setup_loading_ui(mut commands: Commands) {
    spawn_loading_ui(&mut commands);
    // Setup the UI
    // Spawn a Camera to show the UI Text.
    commands.spawn((
        Name::new("Camera"),
        Camera2dBundle::default(),
        IsDefaultUiCamera,
        StateScoped(GameState::Loading),
    ));
}

fn spawn_loading_ui(command: &mut Commands) {
    let ui_root = command
        .spawn((
            Name::new("UI Root"),
            UiRoot,
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Px(10.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            StateScoped(GameState::Loading),
        ))
        .observe(on_asset_loaded)
        .id();

    let loading_text = command
        .spawn((
            Name::new("Header Text"),
            TextBundle::from_section(
                "Loading",
                TextStyle {
                    font_size: 40.0,
                    color: Color::srgb(0.867, 0.827, 0.412),
                    ..default()
                },
            ),
        ))
        .id();

    command.entity(ui_root).push_children(&[loading_text]);
}
