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

pub enum DroppedStatus {
    Dropped,
    NotDropped([u8; 4]),
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
    fn get_cells_from_position(&self, position: &TetrominoPosition) -> [u8; 4] {
        fn handle_i(position: &TetrominoPosition) -> [u8; 4] {
            let row = position.row;
            let col = position.col;
            [
                Tetromino::get_cell_from_row_and_column(row, col),
                Tetromino::get_cell_from_row_and_column(row + 1, col),
                Tetromino::get_cell_from_row_and_column(row + 2, col),
                Tetromino::get_cell_from_row_and_column(row + 3, col),
            ]
        }

        fn handle_t(position: &TetrominoPosition) -> [u8; 4] {
            let row = position.row;
            let col = position.col;
            [
                Tetromino::get_cell_from_row_and_column(row, col),
                Tetromino::get_cell_from_row_and_column(row + 1, col),
                Tetromino::get_cell_from_row_and_column(row, col - 1),
                Tetromino::get_cell_from_row_and_column(row, col + 1),
            ]
        }

        fn handle_j(position: &TetrominoPosition) -> [u8; 4] {
            let row = position.row;
            let col = position.col;
            [
                Tetromino::get_cell_from_row_and_column(row, col),
                Tetromino::get_cell_from_row_and_column(row + 1, col),
                Tetromino::get_cell_from_row_and_column(row + 2, col),
                Tetromino::get_cell_from_row_and_column(row + 2, col - 1),
            ]
        }

        fn handle_l(position: &TetrominoPosition) -> [u8; 4] {
            let row = position.row;
            let col = position.col;
            [
                Tetromino::get_cell_from_row_and_column(row, col),
                Tetromino::get_cell_from_row_and_column(row + 1, col),
                Tetromino::get_cell_from_row_and_column(row + 2, col),
                Tetromino::get_cell_from_row_and_column(row + 2, col + 1),
            ]
        }

        fn handle_o(position: &TetrominoPosition) -> [u8; 4] {
            let row = position.row;
            let col = position.col;
            [
                Tetromino::get_cell_from_row_and_column(row, col),
                Tetromino::get_cell_from_row_and_column(row + 1, col),
                Tetromino::get_cell_from_row_and_column(row, col + 1),
                Tetromino::get_cell_from_row_and_column(row + 1, col + 1),
            ]
        }

        fn handle_s(position: &TetrominoPosition) -> [u8; 4] {
            let row = position.row;
            let col = position.col;
            [
                Tetromino::get_cell_from_row_and_column(row, col),
                Tetromino::get_cell_from_row_and_column(row + 1, col),
                Tetromino::get_cell_from_row_and_column(row, col + 1),
                Tetromino::get_cell_from_row_and_column(row + 1, col - 1),
            ]
        }

        fn handle_z(position: &TetrominoPosition) -> [u8; 4] {
            let row = position.row;
            let col = position.col;
            [
                Tetromino::get_cell_from_row_and_column(row, col),
                Tetromino::get_cell_from_row_and_column(row + 1, col),
                Tetromino::get_cell_from_row_and_column(row, col - 1),
                Tetromino::get_cell_from_row_and_column(row + 1, col + 1),
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

    pub fn get_cell_from_row_and_column(row: u8, col: u8) -> u8 {
        row * game::NUMBER_OF_COLUMNS + col
    }

    pub fn drop_down(
        &mut self,
        board: &[u8; game::NUMBER_OF_ROWS as usize * game::NUMBER_OF_COLUMNS as usize],
    ) -> DroppedStatus {
        if self.position.row + self.tetromino.height() == game::NUMBER_OF_ROWS {
            let cells = self.get_cells();
            return DroppedStatus::NotDropped(cells);
        }

        let next_row = std::cmp::min(
            game::NUMBER_OF_ROWS - self.tetromino.height(),
            self.position.row + 1,
        );

        let next_position = TetrominoPosition { row: next_row, col: self.position.col };
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

    pub fn next<R>(&mut self, rng: &mut R)
    where
        R: Rng + ?Sized,
    {
        self.current = (&self.next).clone();
        self.next = Tetromino::new(rng);
    }

    pub fn get_current_type(&self) -> &TetrominoType {
        &self.current.tetromino
    }

    pub fn get_current_cells(&self) -> [u8; 4] {
        self.current.get_cells()
    }

    pub fn drop_down(
        &mut self,
        board: &[u8; game::NUMBER_OF_ROWS as usize * game::NUMBER_OF_COLUMNS as usize],
    ) -> DroppedStatus {
        self.current.drop_down(&board)
    }
}
