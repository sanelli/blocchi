use bevy::prelude::Resource;
use rand::Rng;

pub mod tetromino;

#[derive(Debug, Resource)]
pub struct GameBoard {
    board: [u8; 10 /* column */ * 20],
    provider: Option<tetromino::TetrominoProvider>,
}

impl GameBoard {
    pub fn new() -> Self
    {
        GameBoard {
            board: [0; 10 * 20],
            provider: None,
        }
    }
    
    pub fn init<R>(&mut self, rng: &mut R)
    where R : Rng + ?Sized
    {
        self.provider = Some(tetromino::TetrominoProvider::new(rng));
    }

    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.board[row * 10 + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: u8) {
        self.board[row * 10 + col] = value;
    }
}
