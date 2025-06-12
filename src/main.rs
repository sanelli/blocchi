mod game;

use bevy::prelude::*;
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::*;
use std::time::Duration;

const CLEAN_UP_OCCUPIED_ROWS_TIME_DELTA_MS : u64 = 5;
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

#[derive(Component)]
struct OccupiedCell;

#[derive(Component)]
struct BorderCell;

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

// TODO : 002. Display points and level (every 10 lines cleared level increases, 10 points for tetromino placed, 100 per line cleared)
// TODO : 003. Tetromino drop faster depending on level
// TODO : 004. Display upcoming tetromino
// TODO : 005. Fast drop down by pressing ⬇️
// TODO : 008. Display Blocchi title on the left upper corner of the screen a asset image
// TODO : 010. Clean code by replacing u8 for cells with usize and by replacing (i8,i8) and (u8,u8) in the code to improve meaning
// TODO : 011. Support game pause when pressing "Space Bar"
// TODO : 012. Sound and background music
// TODO : 013. In pause show menu to restart game, exit, enable/disable sound effects, enable.disable bg music
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<ChaCha8Rng>::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_and_rotate_tetromino,
                drop_tetromino_down,
                paint_tetromino_outline,
            )
                .chain()
                .run_if(in_state(GameStatus::Running)),
        )
        .add_systems(
            Update,
            despawn_filled_up_rows.run_if(in_state(GameStatus::RemovingFilledRows)),
        )
        .add_systems(Update, paint_occupied_cells_outline)
        .add_systems(Update, paint_board_border_outline)
        .insert_resource(game::GameBoard::new())
        .insert_resource(GameSettings {
            descend_timer: Timer::new(Duration::from_millis(200), TimerMode::Repeating),
            last_despawned_cell: None,
            remove_filled_cells_times: Timer::new(Duration::from_millis(CLEAN_UP_OCCUPIED_ROWS_TIME_DELTA_MS), TimerMode::Repeating),
        })
        .init_state::<GameStatus>();
    app.run();
}

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
                BorderCell,
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

fn paint_board_border_outline(
    query : Query<&Transform, With<BorderCell>>,
    mut gizmos: Gizmos) {
    for  transform in query {
        gizmos.rect_2d(
            Isometry2d::from_xy(transform.translation.x, transform.translation.y),
            Vec2::splat(SQUARE_SIZE),
            DARK_GRAY,
        )
    }
}

fn paint_tetromino_outline(
    query: Query<&mut Transform, With<TetrominoCell>>,
    game_board: Res<game::GameBoard>,
    mut gizmos: Gizmos)
{
    let tetromino_type = game_board.get_current_tetromino_type();
    let color = get_tetromino_outline_color_by_type(&tetromino_type);

    for transform in query
    {
        gizmos.rect_2d(
            Isometry2d::from_xy(transform.translation.x, transform.translation.y),
            Vec2::splat(SQUARE_SIZE),
            *color,
        )
    }
}

fn paint_occupied_cells_outline(
    query : Query<&Transform, With<OccupiedCell>>,
    mut gizmos: Gizmos) {

    for transform in query {
        gizmos.rect_2d(
            Isometry2d::from_xy(transform.translation.x, transform.translation.y),
            Vec2::splat(SQUARE_SIZE),
            GRAY,
        )
    }
}

fn move_and_rotate_tetromino(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_board: ResMut<game::GameBoard>,
    mut query: Query<(Entity, &mut Transform), With<TetrominoCell>>,
) {
    let moved;

    if keys.just_released(KeyCode::ArrowRight) {
        moved = game_board.move_tetromino(game::tetromino::MoveDirection::Right);
    } else if keys.just_released(KeyCode::ArrowLeft) {
        moved = game_board.move_tetromino(game::tetromino::MoveDirection::Left);
    } else if keys.just_released(KeyCode::ArrowDown) {
        // TODO: Fast drop down
        moved = game::tetromino::MoveStatus::NotMoved;
    } else if keys.just_released(KeyCode::ArrowUp) {
        moved =  game_board.rotate_tetromino();
    } else {
        moved = game::tetromino::MoveStatus::NotMoved;
    }

    if let game::tetromino::MoveStatus::Moved = moved {
        update_tetromino_position_of_cells(&game_board, &mut query);
    }
}

fn drop_tetromino_down(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<TetrominoCell>>,
    mut game_board: ResMut<game::GameBoard>,
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
                update_tetromino_position_of_cells(&game_board, &mut query);
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

                // Is game-over?
                let can_spawn_more_tetromino = game_board.next_tetromino(&mut rng);
                match can_spawn_more_tetromino {
                    game::tetromino::CanSpawnMoreTetromino::Yes => {
                        // If not-dropped we need to check if any line has been filled up so they can be exploded
                        if let Some(_) = game_board.get_next_cell_from_filled_row_after(None) {
                            game_settings.last_despawned_cell = None;
                            game_settings.remove_filled_cells_times.reset();
                            next_state.set(GameStatus::RemovingFilledRows);
                        } else {
                            do_spawn_tetromino(
                                &mut commands,
                                &mut game_board,
                                materials,
                                shape,
                            );
                        }
                    }
                    game::tetromino::CanSpawnMoreTetromino::No => {
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    // tick the timer
    game_settings.remove_filled_cells_times.tick(time.delta());

    if game_settings.remove_filled_cells_times.just_finished() {
        match game_board.get_next_cell_from_filled_row_after(game_settings.last_despawned_cell) {
            None => {
                // Collapse filled up cells
                game_board.collapse_filled_rows();

                // Despawn all the remaining filled cells
                for (entity, _) in query {
                    commands.entity(entity).despawn();
                }

                // Spawn again all the occupied cells
                let shape = meshes.add(Rectangle::new(SQUARE_SIZE, SQUARE_SIZE));
                for row in 0..game::NUMBER_OF_ROWS
                {
                    for col in 0..game::NUMBER_OF_COLUMNS
                    {
                        let cell = game::tetromino::Tetromino::get_cell_from_row_and_column(row, col);
                        if game_board.is_cell_occupied(cell) {
                            commands.spawn((
                                OccupiedCell,
                                Mesh2d(shape.clone()),
                                MeshMaterial2d(materials.add(DARK_GRAY)),
                                get_transform_by_board_cell(cell),
                            ));
                        }
                    }
                }

                // Spawn tetromino
                let shape = meshes.add(Rectangle::new(SQUARE_SIZE, SQUARE_SIZE));
                do_spawn_tetromino(
                    &mut commands,
                    &mut game_board,
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
                let transformation_of_the_cell_to_despawn =
                    get_transform_by_board_cell(cell_to_despawn);
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
    game_board: &mut ResMut<game::GameBoard>,
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
}

fn update_tetromino_position_of_cells(
    game_board: &ResMut<game::GameBoard>,
    query: &mut Query<(Entity, &mut Transform), With<TetrominoCell>>,
) {
    let cells = game_board.get_current_cells();
    for (index, (_, ref mut transform)) in query.iter_mut().enumerate() {
        let updated_transformation = get_transform_by_board_cell(cells[index]);

        transform.translation.x = updated_transformation.translation.x;
        transform.translation.y = updated_transformation.translation.y;
    }
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
