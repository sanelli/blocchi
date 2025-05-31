mod game;

use bevy::prelude::*;
use bevy_prng::ChaCha8Rng;
use bevy_rand::prelude::*;

fn main() {
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<ChaCha8Rng>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, paint)
        .insert_resource(game::GameBoard::new());
    app.run();
}

const SQUARE_SIZE: f32 = 30.0;

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
    let gray1 = Color::linear_rgb(0.7, 0.7, 0.7);

    for row in 0..22 {
        for col in 0..12 {
            if row != 0 && row != 21 {
                if col != 0 && col != 11 {
                    continue;
                }
            }

            commands.spawn((
                Mesh2d(shape.clone()),
                MeshMaterial2d(materials.add(gray1)),
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
}

fn paint(
    mut gizmos: Gizmos,
    mut game_board: ResMut<game::GameBoard>,
) {
    let gray2 = Color::linear_rgb(0.3, 0.3, 0.3);

    for row in 0..22 {
        for col in 0..12 {
            if row != 0 && row != 21 {
                if col != 0 && col != 11 {
                    continue;
                }
            }

            gizmos.rect_2d(
                Isometry2d::from_xy(
                    SQUARE_SIZE / 2.0 - 6.0 * SQUARE_SIZE + col as f32 * SQUARE_SIZE,
                    SQUARE_SIZE / 2.0 - 11.0 * SQUARE_SIZE + row as f32 * SQUARE_SIZE,
                ),
                Vec2::splat(SQUARE_SIZE),
                gray2,
            )
        }
    }

    game_board.set(1, 2, 4);
}