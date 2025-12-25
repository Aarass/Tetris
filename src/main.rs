use bevy::{prelude::*, window::WindowResolution};

mod consts;
mod matrix;
mod pieces;

use pieces::*;

use crate::{
    consts::{COLS, FALL_SPEED_UP, ROWS, TILE_SIZE},
    matrix::{Matrix, check_for_colision, fix_piece},
};
use rand::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(get_window_settings()))
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .add_systems(Update, (advance_timer, apply_gravity).chain())
        // .add_systems(Update, check_for_collision)
        // .add_systems(Update, bounds)
        .add_systems(Update, create_piece)
        // .add_systems(Update, update_random_field)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
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

    commands.insert_resource(CurrentPieceHolder(None));
    commands.insert_resource(Matrix::try_new(COLS as usize, ROWS as usize).unwrap());
}

fn handle_input(
    input: Res<ButtonInput<KeyCode>>,
    mut holder: ResMut<CurrentPieceHolder>,
    mut query: Query<(&mut Mesh2d, &mut Transform), With<CurrentPieceTag>>,
    mut tick: ResMut<Tick>,
) {
    if input.just_pressed(KeyCode::KeyP) {
        if tick.mult == 0.0 {
            tick.mult = 1.0;
        } else {
            tick.mult = 0.0;
        }
    }

    let Some(current_piece) = holder.0.as_mut() else {
        return;
    };

    let Ok((mut mesh_comp, mut transform)) = query.single_mut() else {
        return;
    };

    if !(input.pressed(KeyCode::KeyZ) && input.pressed(KeyCode::KeyX)) {
        if input.just_pressed(KeyCode::KeyZ) {
            current_piece.rotate_ccw();
        }

        if input.just_pressed(KeyCode::KeyX) {
            current_piece.rotate_cw();
        }

        if input.just_pressed(KeyCode::KeyZ) || input.just_pressed(KeyCode::KeyX) {
            *mesh_comp = Mesh2d(current_piece.get_mesh().to_owned());
        }
    }

    if !(input.pressed(KeyCode::KeyH) && input.pressed(KeyCode::KeyL)) {
        if input.just_pressed(KeyCode::KeyH) {
            move_piece(&mut transform, Direction::Left);
        } else if input.just_pressed(KeyCode::KeyL) {
            move_piece(&mut transform, Direction::Right);
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

fn apply_gravity(
    mut commands: Commands,
    tick: ResMut<Tick>,
    mut query: Query<(Entity, &mut Transform), With<CurrentPieceTag>>,
    mut matrix: ResMut<Matrix>,
    mut piece_holder: ResMut<CurrentPieceHolder>,
) {
    if tick.timer.just_finished() {
        if let Ok((entity, mut transform)) = query.single_mut() {
            move_piece(&mut transform, Direction::Down);

            let piece = piece_holder.0.as_ref().unwrap();

            let piece_table = piece.get_table();
            let piece_indicies = get_piece_indicies(&transform);

            let collided = check_for_colision(&matrix, piece_table, &piece_indicies);

            if collided {
                move_piece(&mut transform, Direction::Up);

                let recalculated_indicies = get_piece_indicies(&transform);
                dbg!(&piece_indicies, &recalculated_indicies);
                fix_piece(&mut matrix, piece_table, &recalculated_indicies);

                print!("{}", matrix.as_ref());

                piece_holder.0 = None;
                commands.entity(entity).remove::<CurrentPieceTag>();
            }
        }
    };
}

#[allow(dead_code)]
fn bounds(
    mut commands: Commands,
    mut holder: ResMut<CurrentPieceHolder>,
    mut query: Query<(Entity, &mut Transform), With<CurrentPieceTag>>,
) {
    if let Some(_) = holder.0.as_ref() {
        if let Ok((entity, transform)) = query.single_mut() {
            if transform.translation.y < -(TILE_SIZE * (ROWS - 3) as f32) {
                holder.0 = None;
                commands.entity(entity).remove::<CurrentPieceTag>();
            }
        }
    }
}

fn create_piece(
    mut commands: Commands,
    mut holder: ResMut<CurrentPieceHolder>,
    mut factory: ResMut<PieceFactory>,
    meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Some(_) = holder.0 {
        return;
    }

    let piece = factory.create_piece(meshes);
    let mesh_handle = piece.get_mesh().to_owned();

    let material = materials.add(get_random_color());

    commands.spawn((
        Mesh2d(mesh_handle),
        MeshMaterial2d(material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        CurrentPieceTag,
    ));

    holder.0 = Some(piece);
}

// fn check_for_collision(
//     tick: ResMut<Tick>,
//     mut query: Query<&mut Transform, With<CurrentPieceTag>>,
// ) {
// }

#[derive(Component)]
struct CurrentPieceTag;

#[derive(Resource)]
struct CurrentPieceHolder(Option<BoxedPiece>);

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
