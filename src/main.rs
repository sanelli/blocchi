mod game;

use crate::game::tetromino::TetrominoType;
use bevy::prelude::*;
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<ChaCha8Rng>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, paint_board_border_outline)
        .add_systems(Update, paint_tetromino_outline)
        .insert_resource(game::GameBoard::new());
    app.run();
}

const SQUARE_SIZE: f32 = 30.0;
const ORANGE: Color = Color::linear_rgb(1.0, 0.647, 0.0);
const RED: Color = Color::linear_rgb(1.0, 0.0, 0.0);
const BLUE: Color = Color::linear_rgb(0.0, 0.0, 1.0);
const DARK_BLUE: Color = Color::linear_rgb(0.0, 0.0, 0.392);
const GREEN: Color = Color::linear_rgb(0.0, 1.0, 0.0);
const DARK_GREEN: Color = Color::linear_rgb(0.0, 0.392, 0.0);
const VIOLET: Color = Color::linear_rgb(0.498, 1.0, 1.0);
const GRAY: Color = Color::linear_rgb(0.7, 0.7, 0.7);
const PINK: Color = Color::linear_rgb(1.0, 0.753, 0.796);
const YELLOW: Color = Color::linear_rgb(1.0, 1.00, 0.00);
const DARK_GRAY: Color = Color::linear_rgb(0.3, 0.3, 0.3);

#[derive(Component)]
struct TetrominoCell;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut config_store: ResMut<GizmoConfigStore>,
    mut rng: GlobalEntropy<ChaCha8Rng>,
    mut game_board: ResMut<game::GameBoard>,
) {
    commands.spawn(Camera2d);

    let shape = meshes.add(Rectangle::new(SQUARE_SIZE, SQUARE_SIZE));

    for row in 0..(game::NUMBER_OF_ROWS + 2) {
        for col in 0..(game::NUMBER_OF_COLUMNS + 2) {
            if row != 0 && row != (game::NUMBER_OF_ROWS + 1) {
                if col != 0 && col != (game::NUMBER_OF_COLUMNS + 1) {
                    continue;
                }
            }

            commands.spawn((
                Mesh2d(shape.clone()),
                MeshMaterial2d(materials.add(GRAY)),
                Transform::from_xyz(
                    SQUARE_SIZE / 2.0 - 6.0 * SQUARE_SIZE + col as f32 * SQUARE_SIZE,
                    SQUARE_SIZE / 2.0 - 11.0 * SQUARE_SIZE + row as f32 * SQUARE_SIZE,
                    0.0,
                ),
            ));
        }
    }

    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line.width = 3.0;

    game_board.init(&mut rng);

    let tetromino_type = game_board.get_current_tetromino_type();
    let color = get_tetromino_color_by_type(&tetromino_type);
    let current_cells = game_board.get_current_cells();

    for tetromino_cell in current_cells {
        commands.spawn((
            TetrominoCell,
            Mesh2d(shape.clone()),
            MeshMaterial2d(materials.add(*color)),
            get_transform_by_board_cell(tetromino_cell)
        ));
    }
}

fn paint_board_border_outline(mut gizmos: Gizmos) {
    for row in 0..(game::NUMBER_OF_ROWS + 2) {
        for col in 0..(game::NUMBER_OF_COLUMNS + 2) {
            if row != 0 && row != (game::NUMBER_OF_ROWS + 1) {
                if col != 0 && col != (game::NUMBER_OF_COLUMNS + 1)  {
                    continue;
                }
            }

            gizmos.rect_2d(
                Isometry2d::from_xy(
                    SQUARE_SIZE / 2.0 - 6.0 * SQUARE_SIZE + col as f32 * SQUARE_SIZE,
                    SQUARE_SIZE / 2.0 - 11.0 * SQUARE_SIZE + row as f32 * SQUARE_SIZE,
                ),
                Vec2::splat(SQUARE_SIZE),
                DARK_GRAY,
            )
        }
    }
}

fn paint_tetromino_outline(mut gizmos: Gizmos, mut game_board: ResMut<game::GameBoard>) {
    let tetromino_type = game_board.get_current_tetromino_type();

    let color = get_tetromino_outline_color_by_type(&tetromino_type);
    let current_cells = game_board.get_current_cells();

    for tetromino_cell in current_cells {
        let transformation =  get_transform_by_board_cell(tetromino_cell);
        gizmos.rect_2d(
            Isometry2d::from_xy(
                transformation.translation.x,
                transformation.translation.y,
            ),
            Vec2::splat(SQUARE_SIZE),
           *color,
        )
    }
}

fn get_tetromino_color_by_type(tetromino_type: &TetrominoType) -> &'static Color {
    match tetromino_type {
        TetrominoType::I => &PINK,
        TetrominoType::O => &GREEN,
        TetrominoType::T => &YELLOW,
        TetrominoType::J => &BLUE,
        TetrominoType::L => &VIOLET,
        TetrominoType::S => &ORANGE,
        TetrominoType::Z => &RED,
    }
}

fn get_tetromino_outline_color_by_type(tetromino_type: &TetrominoType) -> &'static Color {
    match tetromino_type {
        TetrominoType::I => &RED,
        TetrominoType::O => &DARK_GREEN,
        TetrominoType::T => &ORANGE,
        TetrominoType::J => &DARK_BLUE,
        TetrominoType::L => &BLUE,
        TetrominoType::S => &YELLOW,
        TetrominoType::Z => &PINK,
    }
}

fn get_row_and_column_by_cell(cell: u8) -> (u8, u8) {
    (
        cell / game::NUMBER_OF_COLUMNS,
        cell % game::NUMBER_OF_COLUMNS,
    )
}

fn get_transform_by_board_cell(cell: u8) -> Transform {
    let (row, col) = get_row_and_column_by_cell(cell);
    Transform::from_xyz(
        SQUARE_SIZE / 2.0 - 6.0 * SQUARE_SIZE + (col + 1) as f32 * SQUARE_SIZE,
        SQUARE_SIZE / 2.0 - 11.0 * SQUARE_SIZE + (game::NUMBER_OF_ROWS - row) as f32 * SQUARE_SIZE,
        0.0,
    )
}
