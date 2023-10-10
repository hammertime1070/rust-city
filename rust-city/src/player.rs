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
        &PlayerNo,
        &mut Velocity,
        &mut Direction,
    )>,
) {
    for (player_no, mut velocity, mut direction) in &mut query {
        if player_no.0 == 1 && keyboard_input.any_just_released([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D]) {
            velocity.linvel = Vec2::ZERO;
            continue;
        }
        if player_no.0 == 2 && keyboard_input.any_just_released([KeyCode::Up, KeyCode::Left, KeyCode::Down, KeyCode::Right]) {
            velocity.linvel = Vec2::Zero;
            continue;
        }
    }
}