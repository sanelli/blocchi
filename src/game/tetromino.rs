use crate::game;
use rand::Rng;

#[derive(Clone, Debug)]
pub enum TetrominoType {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

#[derive(Clone, Debug)]
pub enum TetrominoRotation {
    Zero,        // 0 degrees
    HalfPi,      // 90 degrees
    Pi,          // 180 degrees
    ThreeHalfPi, // 270 degrees
}

#[derive(Clone, Debug)]
pub struct TetrominoPosition {
    row: u8,
    col: u8,
}

#[derive(Clone, Debug)]
pub struct Tetromino {
    tetromino: TetrominoType,
    position: TetrominoPosition,
    rotation: TetrominoRotation,
}

#[derive(Debug)]
pub struct TetrominoProvider {
    current: Tetromino,
    next: Tetromino,
}

#[derive(Debug)]
pub enum DroppedStatus {
    Dropped,
    NotDropped([u8; 4]),
}

#[derive(Debug, PartialEq, Eq)]
pub enum MoveStatus {
    Moved,
    NotMoved,
}

#[derive(Debug)]
pub enum MoveDirection {
    Right,
    Left,
}

impl TetrominoType {
    fn random<R>(rng: &mut R) -> Self
    where
        R: Rng + ?Sized,
    {
        let random_value = rng.random_range(1..=7);
        match random_value {
            1 => TetrominoType::I,
            2 => TetrominoType::O,
            3 => TetrominoType::T,
            4 => TetrominoType::J,
            5 => TetrominoType::L,
            6 => TetrominoType::S,
            7 => TetrominoType::Z,
            _ => panic!("Invalid tetromino type"),
        }
    }

    // TODO: HANDLE ROTATION
    fn height(&self) -> u8 {
        match self {
            TetrominoType::I => 4,
            TetrominoType::O => 2,
            TetrominoType::T => 2,
            TetrominoType::J => 3,
            TetrominoType::L => 3,
            TetrominoType::S => 2,
            TetrominoType::Z => 2,
        }
    }
}

impl Tetromino {
    fn new<R>(rng: &mut R) -> Self
    where
        R: Rng + ?Sized,
    {
        fn get_starting_column(tetromino_type: &TetrominoType) -> u8 {
            match tetromino_type {
                TetrominoType::I => 4,
                TetrominoType::O => 4,
                TetrominoType::T => 5,
                TetrominoType::J => 5,
                TetrominoType::L => 4,
                TetrominoType::S => 4,
                TetrominoType::Z => 5,
            }
        }

        let tetromino_type = TetrominoType::random(rng);
        let starting_column = get_starting_column(&tetromino_type);

        Self {
            tetromino: tetromino_type,
            position: TetrominoPosition {
                row: 0,
                col: starting_column,
            },
            rotation: TetrominoRotation::Zero,
        }
    }

    fn get_cells(&self) -> [u8; 4] {
        self.get_cells_from_position(&self.position)
    }

    // TODO: Handle rotation
    fn get_cell_positions_from_position(&self, position: &TetrominoPosition) -> [(i8, i8); 4] {
        fn handle_i(position: &TetrominoPosition) -> [(i8, i8); 4] {
            let row = position.row as i8;
            let col = position.col as i8;
            [(row, col), (row + 1, col), (row + 2, col), (row + 3, col)]
        }

        fn handle_t(position: &TetrominoPosition) -> [(i8, i8); 4] {
            let row = position.row as i8;
            let col = position.col as i8;
            [(row, col), (row + 1, col), (row, col - 1), (row, col + 1)]
        }

        fn handle_j(position: &TetrominoPosition) -> [(i8, i8); 4] {
            let row = position.row as i8;
            let col = position.col as i8;
            [
                (row, col),
                (row + 1, col),
                (row + 2, col),
                (row + 2, col - 1),
            ]
        }

        fn handle_l(position: &TetrominoPosition) -> [(i8, i8); 4] {
            let row = position.row as i8;
            let col = position.col as i8;
            [
                (row, col),
                (row + 1, col),
                (row + 2, col),
                (row + 2, col + 1),
            ]
        }

        fn handle_o(position: &TetrominoPosition) -> [(i8, i8); 4] {
            let row = position.row as i8;
            let col = position.col as i8;
            [
                (row, col),
                (row + 1, col),
                (row, col + 1),
                (row + 1, col + 1),
            ]
        }

        fn handle_s(position: &TetrominoPosition) -> [(i8, i8); 4] {
            let row = position.row as i8;
            let col = position.col as i8;
            [
                (row, col),
                (row + 1, col),
                (row, col + 1),
                (row + 1, col - 1),
            ]
        }

        fn handle_z(position: &TetrominoPosition) -> [(i8, i8); 4] {
            let row = position.row as i8;
            let col = position.col as i8;
            [
                (row, col),
                (row + 1, col),
                (row, col - 1),
                (row + 1, col + 1),
            ]
        }

        match self.tetromino {
            TetrominoType::I => handle_i(&position),
            TetrominoType::O => handle_o(&position),
            TetrominoType::T => handle_t(&position),
            TetrominoType::J => handle_j(&position),
            TetrominoType::L => handle_l(&position),
            TetrominoType::S => handle_s(&position),
            TetrominoType::Z => handle_z(&position),
        }
    }

    fn get_cells_from_positions(&self, positions: &[(i8, i8); 4]) -> [u8; 4] {
        [
            Tetromino::get_cell_from_row_and_column(positions[0].0 as u8, positions[0].1 as u8),
            Tetromino::get_cell_from_row_and_column(positions[1].0 as u8, positions[1].1 as u8),
            Tetromino::get_cell_from_row_and_column(positions[2].0 as u8, positions[2].1 as u8),
            Tetromino::get_cell_from_row_and_column(positions[3].0 as u8, positions[3].1 as u8),
        ]
    }

    fn get_cells_from_position(&self, position: &TetrominoPosition) -> [u8; 4] {
        let positions = self.get_cell_positions_from_position(position);
        self.get_cells_from_positions(&positions)
    }

    pub fn get_cell_from_row_and_column(row: u8, col: u8) -> u8 {
        row * game::NUMBER_OF_COLUMNS + col
    }

    fn drop_down(&mut self, board: &[u8; game::NUMBER_OF_CELLS as usize]) -> DroppedStatus {
        if self.position.row + self.tetromino.height() == game::NUMBER_OF_ROWS {
            let cells = self.get_cells();
            return DroppedStatus::NotDropped(cells);
        }

        let next_row = std::cmp::min(
            game::NUMBER_OF_ROWS - self.tetromino.height(),
            self.position.row + 1,
        );

        let next_position = TetrominoPosition {
            row: next_row,
            col: self.position.col,
        };
        let targeted_cells = self.get_cells_from_position(&next_position);

        for target_cell in targeted_cells {
            if board[target_cell as usize] == 1 {
                let cells = self.get_cells();
                return DroppedStatus::NotDropped(cells);
            }
        }

        self.position.row = next_row;
        DroppedStatus::Dropped
    }

    fn move_with_direction(
        &mut self,
        direction: MoveDirection,
        board: &[u8; game::NUMBER_OF_CELLS as usize],
    ) -> MoveStatus {
        let next_column = match direction {
            MoveDirection::Left => self.position.col as i8 - 1,
            MoveDirection::Right => self.position.col as i8 + 1,
        };

        // Compute the next position
        let next_position = TetrominoPosition {
            row: self.position.row,
            col: next_column as u8,
        };

        // Check the tetromino rows and columns are within boundaries
        let cells = self.get_cell_positions_from_position(&next_position);
        for cell in cells {
            if cell.1 < 0 || cell.1  >= game::NUMBER_OF_COLUMNS as i8
            {
                return MoveStatus::NotMoved;
            }
        }

        // Check the tetromino is not crossing any cell already occupied
        let targeted_cells = self.get_cells_from_positions(&cells);
        for target_cell in targeted_cells {
            if board[target_cell as usize] == 1 {
                return MoveStatus::NotMoved;
            }
        }

        // If the above checks are successful, then it means that the tetromino moved!
        self.position.col = next_column as u8;
        MoveStatus::Moved
    }
}

impl TetrominoProvider {
    pub fn new<R>(rng: &mut R) -> Self
    where
        R: Rng + ?Sized,
    {
        Self {
            current: Tetromino::new(rng),
            next: Tetromino::new(rng),
        }
    }

    pub fn next<R>(
        &mut self,
        rng: &mut R,
        board: &[u8; game::NUMBER_OF_CELLS as usize],
    ) -> Option<()>
    where
        R: Rng + ?Sized,
    {
        self.current = (&self.next).clone();
        self.next = Tetromino::new(rng);

        let new_current_cells = self.current.get_cells();
        for cell in new_current_cells {
            if board[cell as usize] == 1 {
                return None;
            }
        }

        Some(())
    }

    pub fn get_current_type(&self) -> &TetrominoType {
        &self.current.tetromino
    }

    pub fn get_current_cells(&self) -> [u8; 4] {
        self.current.get_cells()
    }

    pub fn drop_down(&mut self, board: &[u8; game::NUMBER_OF_CELLS as usize]) -> DroppedStatus {
        self.current.drop_down(&board)
    }

    pub fn move_current(
        &mut self,
        direction: MoveDirection,
        board: &[u8; game::NUMBER_OF_CELLS as usize],
    ) -> MoveStatus {
        self.current.move_with_direction(direction, &board)
    }
}
