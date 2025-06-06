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

pub enum DroppedStatus
{
    Dropped,
    NotDropped
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
        fn get_starting_column(tetromino_type: &TetrominoType) -> u8{
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
            position: TetrominoPosition { row: 0, col: starting_column },
            rotation: TetrominoRotation::Zero,
        }
    }

    // TODO: Handle rotation
    fn get_cells(&self) -> [u8; 4] {
        fn handle_i(tetromino: &Tetromino) -> [u8; 4] {
            let position = &tetromino.position;
            let row = position.row;
            let col = position.col;
            [
                Tetromino::get_cell_from_row_and_column(row, col),
                Tetromino::get_cell_from_row_and_column(row + 1, col),
                Tetromino::get_cell_from_row_and_column(row + 2, col),
                Tetromino::get_cell_from_row_and_column(row + 3, col),
            ]
        }

        fn handle_t(tetromino: &Tetromino) -> [u8; 4] {
            let position = &tetromino.position;
            let row = position.row;
            let col = position.col;
            [
                Tetromino::get_cell_from_row_and_column(row, col),
                Tetromino::get_cell_from_row_and_column(row + 1, col),
                Tetromino::get_cell_from_row_and_column(row, col - 1),
                Tetromino::get_cell_from_row_and_column(row, col + 1),
            ]
        }

        fn handle_j(tetromino: &Tetromino) -> [u8; 4] {
            let position = &tetromino.position;
            let row = position.row;
            let col = position.col;
            [
                Tetromino::get_cell_from_row_and_column(row, col),
                Tetromino::get_cell_from_row_and_column(row + 1, col),
                Tetromino::get_cell_from_row_and_column(row + 2, col),
                Tetromino::get_cell_from_row_and_column(row + 2, col - 1),
            ]
        }

        fn handle_l(tetromino: &Tetromino) -> [u8; 4] {
            let position = &tetromino.position;
            let row = position.row;
            let col = position.col;
            [
                Tetromino::get_cell_from_row_and_column(row, col),
                Tetromino::get_cell_from_row_and_column(row + 1, col),
                Tetromino::get_cell_from_row_and_column(row + 2, col),
                Tetromino::get_cell_from_row_and_column(row + 2, col + 1),
            ]
        }

        fn handle_o(tetromino: &Tetromino) -> [u8; 4] {
            let position = &tetromino.position;
            let row = position.row;
            let col = position.col;
            [
                Tetromino::get_cell_from_row_and_column(row, col),
                Tetromino::get_cell_from_row_and_column(row + 1, col),
                Tetromino::get_cell_from_row_and_column(row, col + 1),
                Tetromino::get_cell_from_row_and_column(row + 1, col + 1),
            ]
        }

        fn handle_s(tetromino: &Tetromino) -> [u8; 4] {
            let position = &tetromino.position;
            let row = position.row;
            let col = position.col;
            [
                Tetromino::get_cell_from_row_and_column(row, col),
                Tetromino::get_cell_from_row_and_column(row + 1, col),
                Tetromino::get_cell_from_row_and_column(row, col + 1),
                Tetromino::get_cell_from_row_and_column(row + 1, col - 1),
            ]
        }

        fn handle_z(tetromino: &Tetromino) -> [u8; 4] {
            let position = &tetromino.position;
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
            TetrominoType::I => handle_i(&self),
            TetrominoType::O => handle_o(&self),
            TetrominoType::T => handle_t(&self),
            TetrominoType::J => handle_j(&self),
            TetrominoType::L => handle_l(&self),
            TetrominoType::S => handle_s(&self),
            TetrominoType::Z => handle_z(&self),
        }
    }

    pub fn get_cell_from_row_and_column(row: u8, col: u8) -> u8 {
        row * game::NUMBER_OF_COLUMNS + col
    }

    // TODO: HANDLE THE STATUS WHEN THE CELL "BELOW" IS ALREADY FILLED
    pub fn drop_down(&mut self, board: &[u8; game::NUMBER_OF_ROWS as usize * game::NUMBER_OF_COLUMNS as usize])
    -> DroppedStatus
    {
        if self.position.row + self.tetromino.height() == game::NUMBER_OF_ROWS
        {
            return DroppedStatus::NotDropped;
        }

        let next_row = std::cmp::min(game::NUMBER_OF_ROWS - self.tetromino.height(), self.position.row + 1);
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

    pub fn drop_down(&mut self, board: &[u8; game::NUMBER_OF_ROWS as usize * game::NUMBER_OF_COLUMNS as usize])
        -> DroppedStatus
    {
        self.current.drop_down(&board)
    }
}
