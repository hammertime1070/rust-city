///----------
/// Constants
///----------

///----------
/// Resources
///----------

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    StartMenu,
    Playing,
    Paused,
    GameOver,
}

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
