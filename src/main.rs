mod game;

use crate::game::GameBoard;
use crate::game::tetromino::CanSpawnMoreTetromino;
use bevy::prelude::*;
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::*;
use std::time::Duration;

#[derive(Component)]
struct TetrominoCell;

#[derive(Component)]
struct OccupiedCell;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameStatus {
    #[default]
    Running,
    RemovingFilledRows,
    GameOver,
}

#[derive(Resource)]
struct GameSettings {
    descend_timer: Timer,
    last_despawned_cell: Option<u8>,
    remove_filled_cells_times: Timer,
}

// TODO: UPDATE THE METHOD TO DRAW THE OUTLINE OF TETROMINO AND FILLED UP CELLS TO QUERY THE MESHES INSTEAD OF DRAWING BLINDLY
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<ChaCha8Rng>::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (move_and_rotate_tetromino, drop_tetromino_down, paint_tetromino_outline)
                .chain()
                .run_if(in_state(GameStatus::Running)))
        .add_systems(Update, despawn_filled_up_rows.run_if(in_state(GameStatus::RemovingFilledRows)))
        .add_systems(Update, paint_occupied_cells_outline)
        .add_systems(Update, paint_board_border_outline)
        .insert_resource(game::GameBoard::new())
        .insert_resource(GameSettings {
            descend_timer: Timer::new(Duration::from_millis(200), TimerMode::Repeating),
            last_despawned_cell: None,
            remove_filled_cells_times: Timer::new(Duration::from_millis(10), TimerMode::Repeating),
        })
        .init_state::<GameStatus>();
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
                get_transform_from_row_and_col(row, col),
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
            get_transform_by_board_cell(tetromino_cell),
        ));
    }
}

fn get_transform_from_row_and_col(row: u8, col: u8) -> Transform {
    Transform::from_xyz(
        SQUARE_SIZE / 2.0 - 6.0 * SQUARE_SIZE + col as f32 * SQUARE_SIZE,
        SQUARE_SIZE / 2.0 - 11.0 * SQUARE_SIZE + row as f32 * SQUARE_SIZE,
        0.0,
    )
}

fn paint_board_border_outline(mut gizmos: Gizmos) {
    for row in 0..(game::NUMBER_OF_ROWS + 2) {
        for col in 0..(game::NUMBER_OF_COLUMNS + 2) {
            if row != 0 && row != (game::NUMBER_OF_ROWS + 1) {
                if col != 0 && col != (game::NUMBER_OF_COLUMNS + 1) {
                    continue;
                }
            }

            let transform = get_transform_from_row_and_col(row, col);
            gizmos.rect_2d(
                Isometry2d::from_xy(transform.translation.x, transform.translation.y),
                Vec2::splat(SQUARE_SIZE),
                DARK_GRAY,
            )
        }
    }
}

fn paint_tetromino_outline(mut gizmos: Gizmos, game_board: ResMut<game::GameBoard>) {
    do_paint_tetromino_outline(&mut gizmos, &game_board);
}

fn do_paint_tetromino_outline(gizmos: &mut Gizmos, game_board: &ResMut<game::GameBoard>) {
    let tetromino_type = game_board.get_current_tetromino_type();

    let color = get_tetromino_outline_color_by_type(&tetromino_type);
    let current_cells = game_board.get_current_cells();

    for tetromino_cell in current_cells {
        let transformation = get_transform_by_board_cell(tetromino_cell);
        gizmos.rect_2d(
            Isometry2d::from_xy(transformation.translation.x, transformation.translation.y),
            Vec2::splat(SQUARE_SIZE),
            *color,
        )
    }
}

fn paint_occupied_cells_outline(mut gizmos: Gizmos, game_board: ResMut<game::GameBoard>) {
    do_paint_occupied_cells_outline(&mut gizmos, &game_board);
}

fn do_paint_occupied_cells_outline(gizmos: &mut Gizmos, game_board: &ResMut<game::GameBoard>) {
    for cell in 0..(game::NUMBER_OF_ROWS * game::NUMBER_OF_COLUMNS) {
        if game_board.is_cell_occupied(cell) {
            let transformation = get_transform_by_board_cell(cell);
            gizmos.rect_2d(
                Isometry2d::from_xy(transformation.translation.x, transformation.translation.y),
                Vec2::splat(SQUARE_SIZE),
                GRAY,
            )
        }
    }
}

fn move_and_rotate_tetromino(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_board: ResMut<game::GameBoard>,
    mut query: Query<(Entity, &mut Transform), With<TetrominoCell>>,
    mut gizmos: Gizmos,
) {
    let mut require_redraw = false;

    if keys.just_released(KeyCode::ArrowRight) {
        let moved = game_board.move_tetromino(game::tetromino::MoveDirection::Right);
        require_redraw = moved == game::tetromino::MoveStatus::Moved;
    } else if keys.just_released(KeyCode::ArrowLeft) {
        let moved = game_board.move_tetromino(game::tetromino::MoveDirection::Left);
        require_redraw = moved == game::tetromino::MoveStatus::Moved;
    } else if keys.just_released(KeyCode::ArrowDown) {
        // TODO: Fast drop down
    } else if keys.just_released(KeyCode::Space) {
        // TODO: Invoke rotate
    }

    if require_redraw {
        do_redraw_tetromino(&game_board, &mut query, &mut gizmos);
    }
}

fn drop_tetromino_down(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<TetrominoCell>>,
    mut game_board: ResMut<game::GameBoard>,
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut game_settings: ResMut<GameSettings>,
    mut rng: GlobalEntropy<ChaCha8Rng>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut next_state: ResMut<NextState<GameStatus>>,
) {
    // tick the timer
    game_settings.descend_timer.tick(time.delta());

    if game_settings.descend_timer.just_finished() {
        let dropped = game_board.drop_down();

        match dropped {
            game::tetromino::DroppedStatus::Dropped => {
                do_redraw_tetromino(&game_board, &mut query, &mut gizmos);
            }
            game::tetromino::DroppedStatus::NotDropped(cells) => {
                // Despawn the current tetromino
                for (entity, _) in query {
                    commands.entity(entity).despawn();
                }

                // Spawn in its place the filled cells blocks
                let shape = meshes.add(Rectangle::new(SQUARE_SIZE, SQUARE_SIZE));
                for cell in cells {
                    commands.spawn((
                        OccupiedCell,
                        Mesh2d(shape.clone()),
                        MeshMaterial2d(materials.add(DARK_GRAY)),
                        get_transform_by_board_cell(cell),
                    ));
                }

                do_paint_occupied_cells_outline(&mut gizmos, &game_board);

                // Is game-over?
                let can_spawn_more_tetromino = game_board.next_tetromino(&mut rng);
                match can_spawn_more_tetromino {
                    CanSpawnMoreTetromino::Yes => {
                        // If not-dropped we need to check if any line has been filled up so they can be exploded
                        if let Some(_) = game_board.get_next_cell_from_filled_row_after(None) {
                            game_settings.last_despawned_cell = None;
                            game_settings.remove_filled_cells_times.reset();
                            next_state.set(GameStatus::RemovingFilledRows);
                        } else {
                            do_spawn_tetromino(
                                &mut commands,
                                &mut game_board,
                                &mut gizmos,
                                materials,
                                shape,
                            );
                        }
                    }
                    CanSpawnMoreTetromino::No => {
                        next_state.set(GameStatus::GameOver);
                    }
                }
            }
        }

        game_settings.descend_timer.reset();
    }
}

fn despawn_filled_up_rows(
    mut commands: Commands,
    query: Query<(Entity, &mut Transform), With<OccupiedCell>>,
    mut game_board: ResMut<game::GameBoard>,
    mut next_state: ResMut<NextState<GameStatus>>,
    mut game_settings: ResMut<GameSettings>,
    mut gizmos: Gizmos,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
)
{
    // tick the timer
    game_settings.remove_filled_cells_times.tick(time.delta());

    if game_settings.remove_filled_cells_times.just_finished() {
        match game_board.get_next_cell_from_filled_row_after(game_settings.last_despawned_cell) {
            None => {
                // Despawn all the remaining filled cells
                // TODO:

                // Spwan again all the occupied cells
                // TODO:

                // Draw outline for filled cells
                // TODO: (INVOKE THE EXISTING METHOD)

                // Spwan tetromino
                let shape = meshes.add(Rectangle::new(SQUARE_SIZE, SQUARE_SIZE));
                do_spawn_tetromino(
                    &mut commands,
                    &mut game_board,
                    &mut gizmos,
                    materials,
                    shape,
                );

                // Reset the last cell to despawn
                game_settings.last_despawned_cell = None;

                // Reset descent timer
                game_settings.descend_timer.reset();

                // Change status
                next_state.set(GameStatus::Running);
            }
            Some(cell_to_despawn) => {
                let transformation_of_the_cell_to_despawn = get_transform_by_board_cell(cell_to_despawn);
                for (entity, transformation) in query {
                    if transformation.eq(&transformation_of_the_cell_to_despawn) {
                        commands.entity(entity).despawn();
                        break;
                    }
                }

                game_settings.last_despawned_cell = Some(cell_to_despawn);
            }
        }

        game_settings.remove_filled_cells_times.reset();
    }
}

fn do_spawn_tetromino(
    commands: &mut Commands,
    game_board: &mut ResMut<GameBoard>,
    mut gizmos: &mut Gizmos,
    mut materials: ResMut<Assets<ColorMaterial>>,
    shape: Handle<Mesh>,
) {
    // Span the new tetromino
    let tetromino_type = game_board.get_current_tetromino_type();
    let color = get_tetromino_color_by_type(&tetromino_type);
    let current_cells = game_board.get_current_cells();

    for tetromino_cell in current_cells {
        commands.spawn((
            TetrominoCell,
            Mesh2d(shape.clone()),
            MeshMaterial2d(materials.add(*color)),
            get_transform_by_board_cell(tetromino_cell),
        ));
    }

    do_paint_tetromino_outline(&mut gizmos, &game_board);
}

fn do_redraw_tetromino(
    game_board: &ResMut<GameBoard>,
    query: &mut Query<(Entity, &mut Transform), With<TetrominoCell>>,
    gizmos: &mut Gizmos,
) {
    let cells = game_board.get_current_cells();
    for (index, (_, ref mut transform)) in query.iter_mut().enumerate() {
        let updated_transformation = get_transform_by_board_cell(cells[index]);

        transform.translation.x = updated_transformation.translation.x;
        transform.translation.y = updated_transformation.translation.y;
    }

    do_paint_tetromino_outline(gizmos, game_board);
}

fn get_tetromino_color_by_type(tetromino_type: &game::tetromino::TetrominoType) -> &'static Color {
    match tetromino_type {
        game::tetromino::TetrominoType::I => &PINK,
        game::tetromino::TetrominoType::O => &GREEN,
        game::tetromino::TetrominoType::T => &YELLOW,
        game::tetromino::TetrominoType::J => &BLUE,
        game::tetromino::TetrominoType::L => &VIOLET,
        game::tetromino::TetrominoType::S => &ORANGE,
        game::tetromino::TetrominoType::Z => &RED,
    }
}

fn get_tetromino_outline_color_by_type(
    tetromino_type: &game::tetromino::TetrominoType,
) -> &'static Color {
    match tetromino_type {
        game::tetromino::TetrominoType::I => &RED,
        game::tetromino::TetrominoType::O => &DARK_GREEN,
        game::tetromino::TetrominoType::T => &ORANGE,
        game::tetromino::TetrominoType::J => &DARK_BLUE,
        game::tetromino::TetrominoType::L => &BLUE,
        game::tetromino::TetrominoType::S => &YELLOW,
        game::tetromino::TetrominoType::Z => &PINK,
    }
}

fn get_transform_by_board_cell(cell: u8) -> Transform {
    let (row, col) = game::tetromino::Tetromino::get_row_and_column_by_cell(cell);
    Transform::from_xyz(
        SQUARE_SIZE / 2.0 - 6.0 * SQUARE_SIZE + (col + 1) as f32 * SQUARE_SIZE,
        SQUARE_SIZE / 2.0 - 11.0 * SQUARE_SIZE + (game::NUMBER_OF_ROWS - row) as f32 * SQUARE_SIZE,
        0.0,
    )
}
