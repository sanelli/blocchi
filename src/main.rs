use bevy::prelude::*;

#[derive(Copy, Clone, Debug)]
struct GameBoard {
    board: [u8; 10 /* column */ * 20],
}

enum TetrominoType {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

enum TetrominoRotation {
    Zero,        // 0 degrees
    HalfPi,      // 90 degrees
    Pi,          // 180 degrees
    ThreeHalfPi, // 270 degrees
}

struct TetrominoPosition {
    row: usize,
    col: usize,
}

struct Tetromino {
    tetromino: TetrominoType,
    position: TetrominoPosition,
    rotation: TetrominoRotation,
}

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins,))
        .add_systems(Startup, setup)
        .add_systems(Update, paint);
    app.run();
}

const SQUARE_SIZE: f32 = 30.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut config_store: ResMut<GizmoConfigStore>,
) {
    commands.spawn(Camera2d);

    let shape = meshes.add(Rectangle::new(SQUARE_SIZE, SQUARE_SIZE));
    let gray1 = Color::linear_rgb(0.7, 0.7, 0.7);

    for row in 0..22 {
        for col in 0..12 {
            if (row != 0 && row != 21) {
                if (col != 0 && col != 11) {
                    continue;
                }
            }

            commands.spawn((
                Mesh2d(shape.clone()),
                MeshMaterial2d(materials.add(gray1)),
                Transform::from_xyz(
                    SQUARE_SIZE / 2.0 - 6.0 * SQUARE_SIZE + col as f32 * SQUARE_SIZE,
                    SQUARE_SIZE / 2.0 -11.0 * SQUARE_SIZE + row as f32 * SQUARE_SIZE,
                    0.0,
                ),
            ));
        }
    }

    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line.width = 3.0;
}

fn paint(mut gizmos: Gizmos) {
    let gray2 = Color::linear_rgb(0.3, 0.3, 0.3);

    for row in 0..22 {
        for col in 0..12 {
            if (row != 0 && row != 21) {
                if (col != 0 && col != 11) {
                    continue;
                }
            }

            gizmos.rect_2d(
                Isometry2d::from_xy(SQUARE_SIZE / 2.0 - 6.0 * SQUARE_SIZE + col as f32 * SQUARE_SIZE,
                                    SQUARE_SIZE / 2.0 -11.0 * SQUARE_SIZE + row as f32 * SQUARE_SIZE,),
                Vec2::splat(SQUARE_SIZE),
                gray2,
            )
        }
    }
}

impl Default for GameBoard {
    fn default() -> Self {
        GameBoard {
            board: [0; 10 * 20],
        }
    }
}

impl Tetromino {
    fn current_occupied_cells(&self) -> [u8; 4] {
        todo!();
    }
}

impl GameBoard {
    fn get(&self, row: usize, col: usize) -> u8 {
        self.board[row * 10 + col]
    }

    fn set(&mut self, row: usize, col: usize, value: u8) {
        self.board[row * 10 + col] = value;
    }
}
