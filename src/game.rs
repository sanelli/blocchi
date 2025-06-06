use crate::game::tetromino::TetrominoType;
use bevy::prelude::*;
use rand::Rng;

pub mod tetromino;

pub const NUMBER_OF_ROWS: u8 = 20;
pub const NUMBER_OF_COLUMNS: u8 = 10;

#[derive(Debug, Resource)]
pub struct GameBoard {
    board: [u8; NUMBER_OF_COLUMNS as usize * NUMBER_OF_ROWS as usize],
    provider: Option<tetromino::TetrominoProvider>,
}

impl GameBoard {
    pub fn new() -> Self
    {
        GameBoard {
            board: [0; NUMBER_OF_ROWS as usize * NUMBER_OF_COLUMNS as usize],
            provider: None,
        }
    }
    
    pub fn init<R>(&mut self, rng: &mut R)
    where R : Rng + ?Sized
    {
        self.provider = Some(tetromino::TetrominoProvider::new(rng));
    }

    pub fn get_current_tetromino_type(&self) -> &TetrominoType
    {
        if let Some(provider) = &self.provider {
            provider.get_current_type()
        } else {
            panic!("Provider has not been initialized.");
        }
    }

    pub fn get_current_cells(&self) -> [u8; 4]
    {
        if let Some(provider) = &self.provider {
            provider.get_current_cells()
        } else {
            panic!("Provider has not been initialized.");
        }
    }

    pub fn drop_down(&mut self)
    {
        if let Some(provider) = &mut self.provider {
            provider.drop_down();
        } else {
            panic!("Provider has not been initialized.");
        }
    }
}
