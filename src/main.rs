mod consts;
mod ecs;
mod game;

use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::*;
use consts::*;
use ecs::*;
use std::time::Duration;

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
        .add_systems(Startup, setup_text_and_scores)
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
        .add_systems(Update, paint_upcoming_tetromino_outline.run_if(in_state(GameStatus::Running)))
        .insert_resource(game::GameBoard::new())
        .insert_resource(GameSettings {
            descend_timer: Timer::new(Duration::from_millis(BASE_SPEED_MS), TimerMode::Repeating),
            last_despawned_cell: None,
            remove_filled_cells_times: Timer::new(
                Duration::from_millis(CLEAN_UP_OCCUPIED_ROWS_TIME_DELTA_MS),
                TimerMode::Repeating,
            ),
            level: 1,
            filled_up_lines: 0,
            score: 0,
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
    let current_cells = game_board.get_current_tetromino_cells();

    for tetromino_cell in current_cells {
        commands.spawn((
            TetrominoCell,
            Mesh2d(shape.clone()),
            MeshMaterial2d(materials.add(*color)),
            get_transform_by_board_cell(tetromino_cell),
        ));
    }

    // Display upcoming tetromino
    let upcoming_type = game_board.get_upcoming_tetromino_type();
    let upcoming_cells = game_board.get_upcoming_tetromino_cells();
    let upcoming_color = get_tetromino_color_by_type(&upcoming_type);

    for upcoming_tetromino_cell in upcoming_cells {
        commands.spawn((
            UpcomingTetrominoCell,
            Mesh2d(shape.clone()),
            MeshMaterial2d(materials.add(*upcoming_color)),
            get_upcoming_tetromino_position_for_cell(upcoming_tetromino_cell),
        ));
    }
}

fn setup_text_and_scores(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_settings: Res<GameSettings>,
) {
    let font = asset_server.load("fonts/NovaSquare-Regular.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 25.0,
        ..default()
    };

    const TEXT_TOP: f32 = 325.00;
    const FIXED_TEXT_X: f32 = 200.00;
    const VARIABLE_TEXT_X: f32 = 300.00;
    const LINE_SIZE: f32 = 30.00;

    commands.spawn((
        Text2d::new("Scores"),
        text_font.clone(),
        TextLayout::new_with_justify(JustifyText::Left),
        Anchor::TopLeft,
        Transform::from_translation(Vec3::new(FIXED_TEXT_X, TEXT_TOP, 0.0)),
    ));

    commands.spawn((
        Text2d::new(game_settings.score.to_string()),
        text_font.clone(),
        TextLayout::new_with_justify(JustifyText::Left),
        Anchor::TopLeft,
        Transform::from_translation(Vec3::new(VARIABLE_TEXT_X, TEXT_TOP, 0.0)),
        TextColor(RED),
        ScoreText,
    ));

    commands.spawn((
        Text2d::new("Level"),
        text_font.clone(),
        TextLayout::new_with_justify(JustifyText::Left),
        Anchor::TopLeft,
        Transform::from_translation(Vec3::new(FIXED_TEXT_X, TEXT_TOP - LINE_SIZE, 0.0)),
    ));

    commands.spawn((
        Text2d::new(game_settings.level.to_string()),
        text_font.clone(),
        TextLayout::new_with_justify(JustifyText::Left),
        Anchor::TopLeft,
        Transform::from_translation(Vec3::new(VARIABLE_TEXT_X, TEXT_TOP - LINE_SIZE, 0.0)),
        TextColor(RED),
        LevelText,
    ));

    commands.spawn((
        Text2d::new("Cleared"),
        text_font.clone(),
        TextLayout::new_with_justify(JustifyText::Left),
        Anchor::TopLeft,
        Transform::from_translation(Vec3::new(FIXED_TEXT_X, TEXT_TOP - LINE_SIZE * 2.00, 0.0)),
    ));

    commands.spawn((
        Text2d::new(game_settings.filled_up_lines.to_string()),
        text_font.clone(),
        TextLayout::new_with_justify(JustifyText::Left),
        Anchor::TopLeft,
        Transform::from_translation(Vec3::new(VARIABLE_TEXT_X, TEXT_TOP - LINE_SIZE * 2.00, 0.0)),
        TextColor(RED),
        ClearedText,
    ));

    commands.spawn((
        Text2d::new("Δms"),
        text_font.clone(),
        TextLayout::new_with_justify(JustifyText::Left),
        Anchor::TopLeft,
        Transform::from_translation(Vec3::new(FIXED_TEXT_X, TEXT_TOP - LINE_SIZE * 3.00, 0.0)),
    ));

    commands.spawn((
        Text2d::new(BASE_SPEED_MS.to_string()),
        text_font.clone(),
        TextLayout::new_with_justify(JustifyText::Left),
        Anchor::TopLeft,
        Transform::from_translation(Vec3::new(VARIABLE_TEXT_X, TEXT_TOP - LINE_SIZE * 3.00, 0.0)),
        TextColor(RED),
        DropDownMsText,
    ));

    commands.spawn((
        Text2d::new("Next"),
        text_font.clone(),
        TextLayout::new_with_justify(JustifyText::Left),
        Anchor::TopLeft,
        Transform::from_translation(Vec3::new(FIXED_TEXT_X, TEXT_TOP - LINE_SIZE * 4.00, 0.0)),
    ));
}

fn get_transform_from_row_and_col(row: u8, col: u8) -> Transform {
    Transform::from_xyz(
        SQUARE_SIZE / 2.0 - 6.0 * SQUARE_SIZE + col as f32 * SQUARE_SIZE,
        SQUARE_SIZE / 2.0 - 11.0 * SQUARE_SIZE + row as f32 * SQUARE_SIZE,
        0.0,
    )
}

fn paint_board_border_outline(query: Query<&Transform, With<BorderCell>>, mut gizmos: Gizmos) {
    for transform in query {
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
    mut gizmos: Gizmos,
) {
    let tetromino_type = game_board.get_current_tetromino_type();
    let color = get_tetromino_outline_color_by_type(&tetromino_type);

    for transform in query {
        gizmos.rect_2d(
            Isometry2d::from_xy(transform.translation.x, transform.translation.y),
            Vec2::splat(SQUARE_SIZE),
            *color,
        )
    }
}

fn paint_upcoming_tetromino_outline(
    query: Query<&mut Transform, With<UpcomingTetrominoCell>>,
    game_board: Res<game::GameBoard>,
    mut gizmos: Gizmos,
) {
    let tetromino_type = game_board.get_upcoming_tetromino_type();
    let color = get_tetromino_outline_color_by_type(&tetromino_type);

    for transform in query {
        gizmos.rect_2d(
            Isometry2d::from_xy(transform.translation.x, transform.translation.y),
            Vec2::splat(SQUARE_SIZE),
            *color,
        )
    }
}

fn paint_occupied_cells_outline(query: Query<&Transform, With<OccupiedCell>>, mut gizmos: Gizmos) {
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
    } else if keys.just_released(KeyCode::ArrowUp) {
        moved = game_board.rotate_tetromino();
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
    upcoming: Query<Entity, (With<UpcomingTetrominoCell>, Without<TetrominoCell>)>,
    mut score_text: Single<&mut Text2d, With<ScoreText>>,
    mut level_text: Single<&mut Text2d, (With<LevelText>, Without<ScoreText>)>,
    mut cleared_text: Single<
        &mut Text2d,
        (With<ClearedText>, Without<LevelText>, Without<ScoreText>),
    >,
    mut drop_down_ms_text: Single<
        &mut Text2d,
        (
            With<DropDownMsText>,
            Without<LevelText>,
            Without<ScoreText>,
            Without<ClearedText>,
        ),
    >,
    mut game_board: ResMut<game::GameBoard>,
    time: Res<Time>,
    mut game_settings: ResMut<GameSettings>,
    mut rng: GlobalEntropy<ChaCha8Rng>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut next_state: ResMut<NextState<GameStatus>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    // tick the timer
    game_settings.descend_timer.tick(time.delta());

    let down_key_pressed = keys.pressed(KeyCode::ArrowDown);
    let timer_just_finished = game_settings.descend_timer.just_finished();

    if timer_just_finished || down_key_pressed {
        let dropped = game_board.drop_down();

        match dropped {
            game::tetromino::DroppedStatus::Dropped => {
                update_tetromino_position_of_cells(&game_board, &mut query);
            }
            game::tetromino::DroppedStatus::NotDropped(cells) => {
                game_settings.score += POINTS_FOR_TETROMINO_DROPPED;

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

                        // Display upcoming tetromino
                        let upcoming_type = game_board.get_upcoming_tetromino_type();
                        let upcoming_cells = game_board.get_upcoming_tetromino_cells();
                        let upcoming_color = get_tetromino_color_by_type(&upcoming_type);

                        for entity in upcoming {
                            commands.entity(entity).despawn();
                        }

                        for upcoming_tetromino_cell in upcoming_cells {
                            commands.spawn((
                                UpcomingTetrominoCell,
                                Mesh2d(shape.clone()),
                                MeshMaterial2d(materials.add(*upcoming_color)),
                                get_upcoming_tetromino_position_for_cell(upcoming_tetromino_cell),
                            ));
                        }

                        // If not-dropped we need to check if any line has been filled up so they can be exploded
                        let number_of_filled_rows = game_board.get_number_of_filled_rows();
                        if number_of_filled_rows > 0 {
                            game_settings.last_despawned_cell = None;
                            game_settings.remove_filled_cells_times.reset();

                            game_settings.score +=
                                number_of_filled_rows as u32 * POINTS_FOR_CLEARED_ROW;
                            game_settings.filled_up_lines += number_of_filled_rows as u32;
                            game_settings.level = std::cmp::min(
                                MAX_LEVEL,
                                game_settings.filled_up_lines as u16 / CLEARED_UP_LINES_PER_LEVEL
                                    + 1,
                            );

                            next_state.set(GameStatus::RemovingFilledRows);
                        } else {
                            do_spawn_tetromino(&mut commands, &mut game_board, materials, shape.clone());
                        }
                    }
                    game::tetromino::CanSpawnMoreTetromino::No => {
                        next_state.set(GameStatus::GameOver);
                        for entity in upcoming {
                            commands.entity(entity).despawn();
                        }
                    }
                }

                // Update the timer
                let mut expected_speed_delta = (game_settings.level - 1) as u64 * LEVEL_SPEED_DELTA;
                if expected_speed_delta > BASE_SPEED_MS - MIN_SPEED_MS {
                    expected_speed_delta = BASE_SPEED_MS - MIN_SPEED_MS;
                }

                let drop_down_ms = BASE_SPEED_MS - expected_speed_delta;
                game_settings.descend_timer =
                    Timer::new(Duration::from_millis(drop_down_ms), TimerMode::Repeating);

                // Update the text messages
                score_text.0 = game_settings.score.to_string();
                cleared_text.0 = game_settings.filled_up_lines.to_string();
                level_text.0 = game_settings.level.to_string();
                drop_down_ms_text.0 = drop_down_ms.to_string();
            }
        }

        if !timer_just_finished {
            game_settings.descend_timer.reset();
        }
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
                for row in 0..game::NUMBER_OF_ROWS {
                    for col in 0..game::NUMBER_OF_COLUMNS {
                        let cell =
                            game::tetromino::Tetromino::get_cell_from_row_and_column(row, col);
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
                do_spawn_tetromino(&mut commands, &mut game_board, materials, shape);

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
    let current_cells = game_board.get_current_tetromino_cells();

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
    let cells = game_board.get_current_tetromino_cells();
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

fn get_upcoming_tetromino_position_for_cell(cell: u8) -> Transform {
    let (row, col) = game::tetromino::Tetromino::get_row_and_column_by_cell(cell);
    Transform::from_xyz(
        280.00 + SQUARE_SIZE / 2.0 - 6.0 * SQUARE_SIZE + (col + 1) as f32 * SQUARE_SIZE,
        SQUARE_SIZE / 2.0 - 11.0 * SQUARE_SIZE + (game::NUMBER_OF_ROWS - row) as f32 * SQUARE_SIZE - 150.00,
        0.0,
    )
}
