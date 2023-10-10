use bevy::prelude::*;
use bevy_rapier2d::*;

///----------
/// Components
///----------

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

///----------
/// Systems
///----------

pub fn players_move(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut Velocity,
        &mut Direction,
    )>,
) {
    for (mut velocity, mut direction) in &mut query {
        if keyboard_input.any_just_released([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D]) {
            velocity.linvel = Vec2::ZERO;
            continue;
        }
        if (keyboard_input.pressed(KeyCode::W)) {
            velocity.linvel = Vec2::new(0.0, PLAYER_SPEED);
            *direction = ::Up;
        }
        else if (keyboard_input.pressed(KeyCode::A)) {
            velocity.linvel = Vec2::new(0.0, PLAYER_SPEED);
            *direction = ::Left;
        }
        else if (keyboard_input.pressed(KeyCode::S)) {
            velocity.linvel = Vec2::new(0.0, PLAYER_SPEED);
            *direction = ::Down;
        }
        else if (keyboard_input.pressed(KeyCode::D)) {
            velocity.linvel = Vec2::new(0.0, PLAYER_SPEED);
            *direction = ::Right;
        }
        else {
            continue;
        }
    }
}