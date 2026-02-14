use bevy::{prelude::*, window::WindowResolution};

mod consts;
mod events;
mod matrix;
mod pieces;

use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use pieces::*;

use crate::{
    consts::{COLS, FALL_SPEED_UP, ROWS, TILE_SIZE},
    events::MovePiece,
    matrix::{Matrix, check_for_colision, fix_piece},
};
use rand::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(get_window_settings()))
        .add_message::<MovePiece>()
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_input, handle_piece_movement))
        .add_systems(Update, (advance_timer, apply_gravity).chain())
        // .add_systems(Update, check_for_collision)
        // .add_systems(Update, bounds)
        .add_systems(Update, create_piece);
    // .add_systems(Update, update_random_field)

    #[cfg(debug)]
    app.add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new());

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        Transform::from_xyz(
            TILE_SIZE * COLS as f32 / 2.0,
            -TILE_SIZE * ROWS as f32 / 2.0,
            0.0,
        ),
    ));

    commands.insert_resource(PieceFactory {});

    commands.insert_resource(Tick {
        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        mult: 1.0,
    });

    commands.insert_resource(Matrix::try_new(COLS as usize, ROWS as usize).unwrap());
}

fn handle_input(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Mesh2d, &mut CurrentPiece)>,
    mut tick: ResMut<Tick>,
    mut message_writer: MessageWriter<MovePiece>,
) {
    if input.just_pressed(KeyCode::KeyP) {
        if tick.mult == 0.0 {
            tick.mult = 1.0;
        } else {
            tick.mult = 0.0;
        }
    }

    let Ok((mut mesh_comp, mut current_piece)) = query.single_mut() else {
        return;
    };

    if !(input.pressed(KeyCode::KeyZ) && input.pressed(KeyCode::KeyX)) {
        if input.just_pressed(KeyCode::KeyZ) {
            current_piece.0.rotate_ccw();
        }

        if input.just_pressed(KeyCode::KeyX) {
            current_piece.0.rotate_cw();
        }

        if input.just_pressed(KeyCode::KeyZ) || input.just_pressed(KeyCode::KeyX) {
            *mesh_comp = Mesh2d(current_piece.0.get_mesh().to_owned());
        }
    }

    if !(input.pressed(KeyCode::KeyH) && input.pressed(KeyCode::KeyL)) {
        if input.just_pressed(KeyCode::KeyH) {
            message_writer.write(MovePiece(Direction::Left));
        } else if input.just_pressed(KeyCode::KeyL) {
            message_writer.write(MovePiece(Direction::Right));
        }
    }
}

fn handle_piece_movement(
    mut commands: Commands,
    mut reader: MessageReader<MovePiece>,
    mut query: Query<(Entity, &mut Transform, &CurrentPiece)>,
    mut matrix: ResMut<Matrix>,
) {
    let Ok((entity, mut transform, current_piece)) = query.single_mut() else {
        return;
    };

    for ev in reader.read() {
        let mut new_position = transform.translation.clone();

        match &ev.0 {
            Direction::Left => {
                new_position -= TILE_SIZE;
            }
            Direction::Right => {
                new_position += TILE_SIZE;
            }
            Direction::Down => {
                new_position -= TILE_SIZE;
            }
            Direction::Up => {
                new_position += TILE_SIZE;
            }
        }

        let piece_table = current_piece.0.get_table();
        let piece_indicies = get_piece_indicies(&new_position);

        let collided = check_for_colision(&matrix, piece_table, &piece_indicies);

        if collided {
            let recalculated_indicies = get_piece_indicies(&transform.translation);
            fix_piece(&mut matrix, piece_table, &recalculated_indicies);
            commands.entity(entity).remove::<CurrentPiece>();

            print!("{}", matrix.as_ref());
        } else {
            transform.translation = new_position;
        }
    }
}

fn advance_timer(time: Res<Time>, mut tick: ResMut<Tick>) {
    let scaled_delta = time.delta().mul_f64(tick.mult);

    tick.timer.tick(scaled_delta);

    // tick.mult += FALL_SPEED_UP;
    // TODO hack za laki pause
    if tick.mult > 0.1 {
        tick.mult += FALL_SPEED_UP;
    }
}

fn apply_gravity(tick: ResMut<Tick>, mut message_writer: MessageWriter<MovePiece>) {
    if tick.timer.just_finished() {
        message_writer.write(MovePiece(Direction::Down));
    };
}

// #[allow(dead_code)]
// fn bounds(mut commands: Commands, mut query: Query<(Entity, &mut Transform), With<CurrentPiece>>) {
//     if let Ok((entity, transform)) = query.single_mut() {
//         if transform.translation.y < -(TILE_SIZE * (ROWS - 3) as f32) {
//             holder.0 = None;
//             commands.entity(entity).remove::<CurrentPiece>();
//         }
//     }
// }

fn create_piece(
    query: Query<&CurrentPiece>,
    mut commands: Commands,
    mut factory: ResMut<PieceFactory>,
    meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok(_) = query.single() {
        return;
    }

    let piece = factory.create_piece(meshes);

    commands.spawn((
        Name::new("Piece"),
        Mesh2d(piece.get_mesh().to_owned()),
        MeshMaterial2d(materials.add(get_random_color())),
        Transform::from_xyz(0.0, 0.0, 0.0),
        CurrentPiece(piece),
    ));
}

// fn check_for_collision(
//     tick: ResMut<Tick>,
//     mut query: Query<&mut Transform, With<CurrentPieceTag>>,
// ) {
// }

#[derive(Component)]
struct CurrentPiece(BoxedPiece);

#[derive(Resource)]
struct PieceFactory();

impl PieceFactory {
    fn create_piece(&mut self, mut meshes: ResMut<Assets<Mesh>>) -> BoxedPiece {
        let mut rng = rand::rng();

        match rng.random_range(0..=5) {
            0 => Box::new(OShape::new(&mut meshes)),
            1 => Box::new(IShape::new(&mut meshes)),
            2 => Box::new(LShape::new(&mut meshes)),
            3 => Box::new(TShape::new(&mut meshes)),
            4 => Box::new(SShape::new(&mut meshes)),
            5 => Box::new(ZShape::new(&mut meshes)),
            _ => unreachable!(),
        }
    }
}

fn get_random_color() -> Color {
    let mut rng = rand::rng();

    Color::linear_rgb(
        rng.random_range(0.0..=1.0),
        rng.random_range(0.0..=1.0),
        rng.random_range(0.0..=1.0),
    )
}

#[derive(Resource)]
struct Tick {
    timer: Timer,
    mult: f64,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

// use rand::prelude::*;
//
// fn update_random_field(mut matrix: ResMut<Matrix>) {
//     let mut rng = rand::rng();
//
//     let col = rng.random_range(0..matrix.height());
//     let row = rng.random_range(0..matrix.width());
//
//     // println!("col: {}, row: {}", col, row);
//     matrix.set(col, row);
//     matrix.clear(0, 0);
// }

// Dodaj ovde "vece" parametre
// Tako da mozes da dodas neku funkciju on_piece_move
// i onda da imas sve sto ti treba da joj prosledis
fn move_piece(transform: &mut Transform, direction: Direction) {
    match direction {
        Direction::Left => {
            transform.translation.x -= TILE_SIZE;
        }
        Direction::Right => {
            transform.translation.x += TILE_SIZE;
        }
        Direction::Down => {
            transform.translation.y -= TILE_SIZE;
        }
        Direction::Up => {
            transform.translation.y += TILE_SIZE;
        }
    }

    // dbg!(get_piece_indicies(&transform));
}

fn get_window_settings() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(TILE_SIZE as u32 * COLS, TILE_SIZE as u32 * ROWS)
                .with_scale_factor_override(1.0),
            ..default()
        }),
        ..default()
    }
}
