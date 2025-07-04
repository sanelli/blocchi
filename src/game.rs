use crate::game::tetromino::{DroppedStatus, MoveDirection, TetrominoType};
use bevy::prelude::*;
use rand::Rng;

pub mod tetromino;

pub const NUMBER_OF_ROWS: u8 = 20;
pub const NUMBER_OF_COLUMNS: u8 = 10;
pub const NUMBER_OF_CELLS: u8 = NUMBER_OF_ROWS * NUMBER_OF_COLUMNS;

#[derive(Debug, Resource)]
pub struct GameBoard {
    board: [u8; NUMBER_OF_COLUMNS as usize * NUMBER_OF_ROWS as usize],
    provider: Option<tetromino::TetrominoProvider>,
}

impl GameBoard {
    pub fn new() -> Self {
        GameBoard {
            board: [0; NUMBER_OF_CELLS as usize],
            provider: None,
        }
    }

    pub fn init<R>(&mut self, rng: &mut R)
    where
        R: Rng + ?Sized,
    {
        if self.provider.is_none() {
            self.provider = Some(tetromino::TetrominoProvider::new(rng));
        }
    }

    pub fn next_tetromino<R>(&mut self, rng: &mut R) -> tetromino::CanSpawnMoreTetromino
    where
        R: Rng + ?Sized,
    {
        if let Some(provider) = &mut self.provider {
            provider.next(rng, &self.board)
        } else {
            panic!("Provider has not been initialized.");
        }
    }

    pub fn reset<R>(&mut self, rng: &mut R)
    where
        R: Rng + ?Sized,
    {
        if let Some(provider) = &mut self.provider {
            self.board = [0; NUMBER_OF_ROWS as usize * NUMBER_OF_COLUMNS as usize];
            provider.next(rng, &self.board);
            provider.next(rng, &self.board);
        } else {
            panic!("Provider has not been initialized.");
        }
    }

    pub fn get_current_tetromino_type(&self) -> &TetrominoType {
        if let Some(provider) = &self.provider {
            provider.get_current_tetromino_type()
        } else {
            panic!("Provider has not been initialized.");
        }
    }

    pub fn get_upcoming_tetromino_type(&self) -> &TetrominoType {
        if let Some(provider) = &self.provider {
            provider.get_upcoming_tetromino_type()
        } else {
            panic!("Provider has not been initialized.");
        }
    }

    pub fn get_current_tetromino_cells(&self) -> [u8; 4] {
        if let Some(provider) = &self.provider {
            provider.get_current_tetromino_cells()
        } else {
            panic!("Provider has not been initialized.");
        }
    }

    pub fn get_upcoming_tetromino_cells(&self) -> [u8; 4] {
        if let Some(provider) = &self.provider {
            provider.get_upcoming_tetromino_cells()
        } else {
            panic!("Provider has not been initialized.");
        }
    }

    pub fn drop_down(&mut self) -> DroppedStatus {
        if let Some(provider) = &mut self.provider {
            let dropped_status = provider.drop_down(&self.board);

            match dropped_status {
                DroppedStatus::NotDropped(occupied_cells) => {
                    for cell in occupied_cells {
                        self.board[cell as usize] = 1;
                    }
                }
                _ => {}
            }

            dropped_status
        } else {
            panic!("Provider has not been initialized.");
        }
    }

    pub fn is_cell_occupied(&self, cell: u8) -> bool {
        self.board[cell as usize] != 0
    }

    pub fn move_tetromino(&mut self, direction: MoveDirection) -> tetromino::MoveStatus {
        if let Some(provider) = &mut self.provider {
            provider.move_current(direction, &self.board)
        } else {
            panic!("Provider has not been initialized.");
        }
    }

    pub fn get_next_cell_from_filled_row_after(&self, cell: Option<u8>) -> Option<u8> {
        let (max_row, mut max_col, mut increment) = match cell {
            Some(cell) => {
                let (r, c) = tetromino::Tetromino::get_row_and_column_by_cell(cell);
                (r, c, true)
            }
            None => (NUMBER_OF_ROWS - 1, 0, false),
        };

        for row in (0..=max_row).rev() {
            if self.is_row_filled(row) {
                if max_col < (NUMBER_OF_COLUMNS - 1) {
                    let col = max_col + (if increment { 1 } else { 0 });
                    return Some(tetromino::Tetromino::get_cell_from_row_and_column(row, col));
                } else {
                    // Being at the end of the col I move to be at the beginning again
                    max_col = 0;
                    increment = false;
                }
            }
        }

        None
    }

    fn get_row_cells(row: u8) -> [u8; NUMBER_OF_COLUMNS as usize] {
        let mut result: [u8; NUMBER_OF_COLUMNS as usize] = [0; NUMBER_OF_COLUMNS as usize];
        result[0] = tetromino::Tetromino::get_cell_from_row_and_column(row, 0);
        for col in 1..NUMBER_OF_COLUMNS {
            result[col as usize] = result[0] + col;
        }
        result
    }

    fn is_row_filled(&self, row: u8) -> bool {
        for cell in Self::get_row_cells(row) {
            if !self.is_cell_occupied(cell) {
                return false;
            }
        }

        true
    }

    pub fn get_number_of_filled_rows(&self) -> u8 {
        let mut number_of_filled_rows = 0;
        for row in 0..NUMBER_OF_ROWS {
            if self.is_row_filled(row) {
                number_of_filled_rows += 1;
            }
        }

        number_of_filled_rows
    }

    pub fn collapse_filled_rows(&mut self) {
        for row in (0..NUMBER_OF_ROWS).rev() {
            while self.is_row_filled(row) {
                // Just in case the row 0 is filled
                if row > 0 {
                    // Drop all the rows, including the one filled
                    for row_to_drop in (1..=row).rev() {
                        for col in 0..NUMBER_OF_COLUMNS {
                            let target_cell = tetromino::Tetromino::get_cell_from_row_and_column(
                                row_to_drop,
                                col,
                            ) as usize;
                            let source_cell = tetromino::Tetromino::get_cell_from_row_and_column(
                                row_to_drop - 1,
                                col,
                            ) as usize;
                            self.board[target_cell] = self.board[source_cell];
                        }
                    }
                }

                // Empty the first row
                for col in 0..NUMBER_OF_COLUMNS {
                    self.board[col as usize] = 0;
                }
            }
        }
    }

    pub fn rotate_tetromino(&mut self) -> tetromino::MoveStatus {
        if let Some(provider) = &mut self.provider {
            provider.rotate_current(&self.board)
        } else {
            panic!("Provider has not been initialized.");
        }
    }
}
