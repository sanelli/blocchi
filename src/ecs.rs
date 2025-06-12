use bevy::prelude::{Component, Resource, States, Timer};

#[derive(Component)]
pub struct TetrominoCell;

#[derive(Component)]
pub struct OccupiedCell;

#[derive(Component)]
pub struct BorderCell;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameStatus {
    #[default]
    Running,
    RemovingFilledRows,
    GameOver,
}

#[derive(Resource)]
pub struct GameSettings {
    pub descend_timer: Timer,
    pub last_despawned_cell: Option<u8>,
    pub remove_filled_cells_times: Timer,
}