/// Declaring our modules
// mod area;
// mod bullet;
// mod common;
// mod enemy;
// mod level;
mod player;
// mod ui;

/// Using our modules so we can reference them without prefix
// use area::*;
// use bullet::*;
// use common::*;
// use enemy::*;
// use level::*;
use player::*;
// use ui::*;

/// use some bevy plugins
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

/// Setting up our main function
fn main () {
    App.new()
    /// Application logic
    .add_systems(
        Startup,
        (
            setup_camera,
            setup_rapier,
        )
    )
    add_systems(
        Update,
        (
            players_move,
        )
        .run_if(in_state(AppState::Playing)),
    )
    .run();
}

/// simple system for setting up the camera
/// TODO: Move to separate file
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

/// simple system for setting up rapier physics
fn setup_rapier(mut rapier_config: ResMut<RapierConfiguration) {
    /// Setting no gravity as we are making a top down game
    rapier_config.gravity = Vec2::ZERO;
}