use bevy::prelude::{Component, Resource, States, Timer};

#[derive(Component)]
pub struct TetrominoCell;

#[derive(Component)]
pub struct UpcomingTetrominoCell;

#[derive(Component)]
pub struct OccupiedCell;

#[derive(Component)]
pub struct BorderCell;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct LevelText;

#[derive(Component)]
pub struct ClearedText;

#[derive(Component)]
pub struct DropDownMsText;

#[derive(Component)]
pub struct PausedText;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameStatus {
    #[default]
    Running,
    RemovingFilledRows,
    GameOver,
    Pause,
}

#[derive(Resource)]
pub struct GameSettings {
    pub descend_timer: Timer,
    pub last_despawned_cell: Option<u8>,
    pub remove_filled_cells_times: Timer,
    pub level: u16,
    pub filled_up_lines: u32,
    pub score: u32,
    pub last_status: Option<GameStatus>,
}
