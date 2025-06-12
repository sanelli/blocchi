use crate::game;
use rand::Rng;
use std::fmt::{Display, Formatter};

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

#[derive(Debug)]
pub enum CanSpawnMoreTetromino {
    Yes,
    No,
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

    fn next_rotation(&self, rotation: &TetrominoRotation) -> TetrominoRotation {
        match self {
            TetrominoType::I => match rotation {
                TetrominoRotation::Zero => TetrominoRotation::HalfPi,
                TetrominoRotation::HalfPi => TetrominoRotation::Zero,
                _ => panic!(
                    "Type '{0}' does not support rotation '{1}'!",
                    &self, rotation
                ),
            },
            TetrominoType::O => match rotation {
                TetrominoRotation::Zero => TetrominoRotation::Zero,
                _ => panic!(
                    "Type '{0}' does not support rotation '{1}'!",
                    &self, rotation
                ),
            },
            TetrominoType::T => match rotation {
                TetrominoRotation::Zero => TetrominoRotation::HalfPi,
                TetrominoRotation::HalfPi => TetrominoRotation::Pi,
                TetrominoRotation::Pi => TetrominoRotation::ThreeHalfPi,
                TetrominoRotation::ThreeHalfPi => TetrominoRotation::Zero,
            },
            TetrominoType::J => match rotation {
                TetrominoRotation::Zero => TetrominoRotation::HalfPi,
                TetrominoRotation::HalfPi => TetrominoRotation::Pi,
                TetrominoRotation::Pi => TetrominoRotation::ThreeHalfPi,
                TetrominoRotation::ThreeHalfPi => TetrominoRotation::Zero,
            },
            TetrominoType::L => match rotation {
                TetrominoRotation::Zero => TetrominoRotation::HalfPi,
                TetrominoRotation::HalfPi => TetrominoRotation::Pi,
                TetrominoRotation::Pi => TetrominoRotation::ThreeHalfPi,
                TetrominoRotation::ThreeHalfPi => TetrominoRotation::Zero,
            },
            TetrominoType::S => match rotation {
                TetrominoRotation::Zero => TetrominoRotation::HalfPi,
                TetrominoRotation::HalfPi => TetrominoRotation::Zero,
                _ => panic!(
                    "Type '{0}' does not support rotation '{1}'!",
                    &self, rotation
                ),
            },
            TetrominoType::Z => match rotation {
                TetrominoRotation::Zero => TetrominoRotation::HalfPi,
                TetrominoRotation::HalfPi => TetrominoRotation::Zero,
                _ => panic!(
                    "Type '{0}' does not support rotation '{1}'!",
                    &self, rotation
                ),
            },
        }
    }

    // Gets the height wrt the current position
    fn height(&self, rotation: &TetrominoRotation) -> u8 {
        match self {
            TetrominoType::I => match rotation {
                TetrominoRotation::Zero => 4,
                TetrominoRotation::HalfPi => 1,
                _ => panic!(
                    "Type '{0}' does not support rotation '{1}'!",
                    &self, rotation
                ),
            },
            TetrominoType::O => 2,
            TetrominoType::T => match rotation {
                TetrominoRotation::Zero => 2,
                TetrominoRotation::HalfPi => 2,
                TetrominoRotation::Pi => 1,
                TetrominoRotation::ThreeHalfPi => 2,
            },
            TetrominoType::J => match rotation {
                TetrominoRotation::Zero => 3,
                TetrominoRotation::HalfPi => 2,
                TetrominoRotation::Pi => 1,
                TetrominoRotation::ThreeHalfPi => 1,
            },
            TetrominoType::L => match rotation {
                TetrominoRotation::Zero => 3,
                TetrominoRotation::HalfPi => 1,
                TetrominoRotation::Pi => 1,
                TetrominoRotation::ThreeHalfPi => 2,
            },
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
        self.get_cells_from_position(&self.position, &self.rotation)
    }

    fn get_cell_positions_from_position(
        &self,
        position: &TetrominoPosition,
        rotation: &TetrominoRotation,
    ) -> [(i8, i8); 4] {
        fn handle_i(position: &TetrominoPosition, rotation: &TetrominoRotation) -> [(i8, i8); 4] {
            let row = position.row as i8;
            let col = position.col as i8;
            match rotation {
                TetrominoRotation::Zero => {
                    [(row, col), (row + 1, col), (row + 2, col), (row + 3, col)]
                }
                TetrominoRotation::HalfPi => {
                    [(row, col), (row, col + 1), (row, col + 2), (row, col + 3)]
                }
                _ => panic!(
                    "Type '{0}' does not support rotation '{1}'!",
                    TetrominoType::I,
                    rotation
                ),
            }
        }

        fn handle_t(position: &TetrominoPosition, rotation: &TetrominoRotation) -> [(i8, i8); 4] {
            let row = position.row as i8;
            let col = position.col as i8;
            match rotation {
                TetrominoRotation::Zero => {
                    [(row, col), (row + 1, col), (row, col - 1), (row, col + 1)]
                }
                TetrominoRotation::HalfPi => {
                    [(row, col), (row + 1, col), (row - 1, col), (row, col + 1)]
                }
                TetrominoRotation::Pi => {
                    [(row, col), (row - 1, col), (row, col - 1), (row, col + 1)]
                }
                TetrominoRotation::ThreeHalfPi => {
                    [(row, col), (row + 1, col), (row - 1, col), (row, col - 1)]
                }
            }
        }

        fn handle_j(position: &TetrominoPosition, rotation: &TetrominoRotation) -> [(i8, i8); 4] {
            let row = position.row as i8;
            let col = position.col as i8;
            match rotation {
                TetrominoRotation::Zero => {
                    [
                        (row, col),
                        (row + 1, col),
                        (row + 2, col),
                        (row + 2, col - 1),
                    ]
                }
                TetrominoRotation::HalfPi => {
                    [
                        (row, col),
                        (row, col + 1),
                        (row, col + 2),
                        (row + 1, col + 2),
                    ]
                }
                TetrominoRotation::Pi => {
                    [
                        (row, col),
                        (row - 1, col),
                        (row - 2, col),
                        (row - 2, col + 1),
                    ]
                }
                TetrominoRotation::ThreeHalfPi => {
                    [
                        (row, col),
                        (row, col - 1),
                        (row, col - 2),
                        (row - 1, col - 2),
                    ]
                }
            }
        }

        fn handle_l(position: &TetrominoPosition, rotation: &TetrominoRotation) -> [(i8, i8); 4] {
            let row = position.row as i8;
            let col = position.col as i8;
            match rotation {
                TetrominoRotation::Zero => {
                    [
                        (row, col),
                        (row + 1, col),
                        (row + 2, col),
                        (row + 2, col + 1),
                    ]
                }
                TetrominoRotation::HalfPi => {
                    [
                        (row, col),
                        (row , col + 1),
                        (row , col + 2),
                        (row - 1, col + 2),
                    ]
                }
                TetrominoRotation::Pi => {
                    [
                        (row, col),
                        (row - 1, col),
                        (row - 2, col),
                        (row - 2, col - 1),
                    ]
                }
                TetrominoRotation::ThreeHalfPi => {
                    [
                        (row, col),
                        (row , col - 1),
                        (row , col - 2),
                        (row + 1, col - 2),
                    ]
                }
            }
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

        fn handle_s(position: &TetrominoPosition, rotation: &TetrominoRotation) -> [(i8, i8); 4] {
            let row = position.row as i8;
            let col = position.col as i8;

            match rotation {
                TetrominoRotation::Zero => [
                    (row, col),
                    (row + 1, col),
                    (row, col + 1),
                    (row + 1, col - 1),
                ],
                TetrominoRotation::HalfPi => [
                    (row, col),
                    (row - 1, col),
                    (row, col + 1),
                    (row + 1, col + 1),
                ],
                _ => panic!(
                    "Type '{0}' does not support rotation '{1}'!",
                    TetrominoType::S,
                    rotation
                ),
            }
        }

        fn handle_z(position: &TetrominoPosition, rotation: &TetrominoRotation) -> [(i8, i8); 4] {
            let row = position.row as i8;
            let col = position.col as i8;

            match rotation {
                TetrominoRotation::Zero => [
                    (row, col),
                    (row + 1, col),
                    (row, col - 1),
                    (row + 1, col + 1),
                ],
                TetrominoRotation::HalfPi => [
                    (row, col),
                    (row - 1, col),
                    (row, col - 1),
                    (row + 1, col - 1),
                ],
                _ => panic!(
                    "Type '{0}' does not support rotation '{1}'!",
                    TetrominoType::S,
                    rotation
                ),
            }
        }

        match self.tetromino {
            TetrominoType::I => handle_i(&position, &rotation),
            TetrominoType::O => handle_o(&position),
            TetrominoType::T => handle_t(&position, &rotation),
            TetrominoType::J => handle_j(&position, &rotation),
            TetrominoType::L => handle_l(&position, &rotation),
            TetrominoType::S => handle_s(&position, &rotation),
            TetrominoType::Z => handle_z(&position, &rotation),
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

    fn get_cells_from_position(
        &self,
        position: &TetrominoPosition,
        rotation: &TetrominoRotation,
    ) -> [u8; 4] {
        let positions = self.get_cell_positions_from_position(position, rotation);
        self.get_cells_from_positions(&positions)
    }

    pub fn get_cell_from_row_and_column(row: u8, col: u8) -> u8 {
        row * game::NUMBER_OF_COLUMNS + col
    }

    pub fn get_row_and_column_by_cell(cell: u8) -> (u8, u8) {
        (
            cell / game::NUMBER_OF_COLUMNS,
            cell % game::NUMBER_OF_COLUMNS,
        )
    }

    fn drop_down(&mut self, board: &[u8; game::NUMBER_OF_CELLS as usize]) -> DroppedStatus {
        if self.position.row + self.tetromino.height(&self.rotation) == game::NUMBER_OF_ROWS {
            let cells = self.get_cells();
            return DroppedStatus::NotDropped(cells);
        }

        let next_row = std::cmp::min(
            game::NUMBER_OF_ROWS - self.tetromino.height(&self.rotation),
            self.position.row + 1,
        );

        let next_position = TetrominoPosition {
            row: next_row,
            col: self.position.col,
        };
        let targeted_cells = self.get_cells_from_position(&next_position, &self.rotation);

        for target_cell in targeted_cells {
            if board[target_cell as usize] != 0 {
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

        let moved =
            self.check_position_and_rotation_are_sound(&next_position, &self.rotation, &board);

        // If the above checks are successful, then it means that the tetromino moved!
        if let MoveStatus::Moved = moved {
            self.position.col = next_column as u8;
        }

        moved
    }

    fn rotate(&mut self, board: &[u8; game::NUMBER_OF_CELLS as usize]) -> MoveStatus {
        // Get the next potential rotation
        let next_rotation = self.tetromino.next_rotation(&self.rotation);
        let moved =
            self.check_position_and_rotation_are_sound(&self.position, &next_rotation, &board);

        // If the above checks are successful, then it means that the tetromino moved!
        if let MoveStatus::Moved = moved {
            self.rotation = next_rotation;
        }

        moved
    }

    fn check_position_and_rotation_are_sound(
        &self,
        next_position: &TetrominoPosition,
        next_rotation: &TetrominoRotation,
        board: &[u8; game::NUMBER_OF_CELLS as usize],
    ) -> MoveStatus {
        // Check the tetromino rows and columns are within boundaries
        let cells = self.get_cell_positions_from_position(&next_position, &next_rotation);
        for cell in cells {
            let col = cell.1;
            let row = cell.0;
            if row < 0
                || row >= game::NUMBER_OF_ROWS as i8
                || col < 0
                || col >= game::NUMBER_OF_COLUMNS as i8
            {
                return MoveStatus::NotMoved;
            }
        }

        // Check the tetromino is not crossing any cell already occupied
        let targeted_cells = self.get_cells_from_positions(&cells);
        for target_cell in targeted_cells {
            if board[target_cell as usize] != 0 {
                return MoveStatus::NotMoved;
            }
        }

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
    ) -> CanSpawnMoreTetromino
    where
        R: Rng + ?Sized,
    {
        self.current = (&self.next).clone();
        self.next = Tetromino::new(rng);

        let new_current_cells = self.current.get_cells();
        for cell in new_current_cells {
            if board[cell as usize] != 0 {
                return CanSpawnMoreTetromino::No;
            }
        }

        CanSpawnMoreTetromino::Yes
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

    pub fn rotate_current(&mut self, board: &[u8; game::NUMBER_OF_CELLS as usize]) -> MoveStatus {
        self.current.rotate(&board)
    }
}

impl Display for TetrominoType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            TetrominoType::I => write!(f, "I"),
            TetrominoType::O => write!(f, "O"),
            TetrominoType::T => write!(f, "T"),
            TetrominoType::J => write!(f, "J"),
            TetrominoType::L => write!(f, "L"),
            TetrominoType::S => write!(f, "S"),
            TetrominoType::Z => write!(f, "Z"),
        }
    }
}

impl Display for TetrominoRotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            TetrominoRotation::Zero => write!(f, "0°"),
            TetrominoRotation::HalfPi => write!(f, "90°"),
            TetrominoRotation::Pi => write!(f, "180°"),
            TetrominoRotation::ThreeHalfPi => write!(f, "270°"),
        }
    }
}
