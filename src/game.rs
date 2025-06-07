use crate::game::tetromino::{DroppedStatus, MoveDirection, TetrominoType};
use bevy::prelude::*;
use rand::Rng;

pub mod tetromino;

pub const NUMBER_OF_ROWS: u8 = 20;
pub const NUMBER_OF_COLUMNS: u8 = 10;

pub const NUMBER_OF_CELLS : u8 = NUMBER_OF_ROWS * NUMBER_OF_COLUMNS;

#[derive(Debug, Resource)]
pub struct GameBoard {
    board: [u8; NUMBER_OF_COLUMNS as usize * NUMBER_OF_ROWS as usize],
    provider: Option<tetromino::TetrominoProvider>,
}

impl GameBoard {
    pub fn new() -> Self
    {
        GameBoard {
            board: [0; NUMBER_OF_CELLS as usize],
            provider: None,
        }
    }
    
    pub fn init<R>(&mut self, rng: &mut R)
    where R : Rng + ?Sized
    {
        if self.provider.is_none() {
            self.provider = Some(tetromino::TetrominoProvider::new(rng));
        }
    }

    pub fn next_tetromino<R>(&mut self,  rng: &mut R) -> Option<()>
    where R : Rng + ?Sized
    {
        if let Some(provider) = &mut self.provider {
            provider.next(rng, &self.board)
        } else {
            panic!("Provider has not been initialized.");
        }
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
        -> DroppedStatus
    {
        if let Some(provider) = &mut self.provider {
            // TODO: IF NOT DROPPED UPDATE THE BOARD WITH THE STATUS OF CURRENT CELLS (SHOULD RETURN THE CELLS AS WELL)
            let dropped_status = provider.drop_down(&self.board);

            match dropped_status {
                DroppedStatus::NotDropped(occupied_cells) =>
                    {
                        for cell in occupied_cells
                        {
                            self.board[cell as usize] = 1;
                        }
                    }
                _ => { }
            }

            dropped_status
        } else {
            panic!("Provider has not been initialized.");
        }
    }

    pub fn is_cell_occupied(&self, cell: u8) -> bool
    {
        self.board[cell as usize] == 1
    }

    pub fn move_tetromino(&mut self, direction : MoveDirection) -> tetromino::MoveStatus
    {
        if let Some(provider) = &mut self.provider {
            provider.move_current(direction, &self.board)
        } else {
            panic!("Provider has not been initialized.");
        }
    }
}
