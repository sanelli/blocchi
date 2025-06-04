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
    row: usize,
    col: usize,
}

#[derive(Clone, Debug)]
pub struct Tetromino {
    tetromino: TetrominoType,
    position: TetrominoPosition,
    rotation: TetrominoRotation,
}

#[derive(Debug)]
pub struct TetrominoProvider
{
    current: Tetromino,
    next: Tetromino,
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
}

impl Tetromino {
    fn new<R>(rng: &mut R) -> Self
    where
        R: Rng + ?Sized,
    {
        Self {
            tetromino: TetrominoType::random(rng),
            position: TetrominoPosition { row: 0, col: 4 },
            rotation: TetrominoRotation::Zero,
        }
    }

    fn get_cells(&self) -> [u8; 4]
    {
        // TODO : GET THE CORRECT CELLS DEPENDING ON TETROMINO, POSITION, ROTATION
        let mut result : [u8; 4] = [0,1,2,3];
        return result;
    }
}

impl TetrominoProvider {
    pub fn new<R>(rng: &mut R) -> Self
    where R : Rng + ?Sized
    {
        Self {
            current: Tetromino::new(rng),
            next: Tetromino::new(rng),
        }
    }

    pub fn next<R>(&mut self, rng: &mut R)
    where R : Rng + ?Sized
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
}
