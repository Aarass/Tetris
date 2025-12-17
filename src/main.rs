use bevy::{prelude::*, window::WindowResolution};

mod consts;
mod pieces;
use pieces::*;

use crate::consts::{COLS, FALL_SPEED_UP, ROWS, TILE_SIZE};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(get_window_settings()))
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .add_systems(Update, (advance_timer, apply_gravity).chain())
        .add_systems(Update, bounds)
        // .add_systems(Update, update_random_field)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(
            TILE_SIZE * COLS as f32 / 2.0,
            -TILE_SIZE * ROWS as f32 / 2.0,
            0.0,
        ),
    ));

    commands.insert_resource(Tick {
        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        mult: 1.0,
    });

    let current_shape = LShape::new(&mut meshes);
    let mesh_handle = current_shape.get_mesh().to_owned();

    commands.insert_resource(CurrentShape(Box::new(current_shape)));

    let material = materials.add(Color::linear_rgb(1.0, 0.3, 0.1));

    commands.spawn((
        Mesh2d(mesh_handle),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        CurrentShapeTag,
    ));

    // let matrix = Matrix::try_new(10, 20).unwrap();
    // commands.insert_resource(matrix);
}

fn handle_input(
    input: Res<ButtonInput<KeyCode>>,
    mut shape: ResMut<CurrentShape>,
    mut query: Query<&mut Mesh2d, With<CurrentShapeTag>>,
) {
    if input.pressed(KeyCode::KeyZ) && input.pressed(KeyCode::KeyX) {
        return;
    }

    if input.just_pressed(KeyCode::KeyZ) {
        shape.0.rotate_ccw();
    }

    if input.just_pressed(KeyCode::KeyX) {
        shape.0.rotate_cw();
    }

    if input.just_pressed(KeyCode::KeyZ) || input.just_pressed(KeyCode::KeyX) {
        let mesh_handle = shape.0.get_mesh();

        let mut mesh_comp = query.single_mut().unwrap();
        *mesh_comp = Mesh2d(mesh_handle.to_owned());
    }
}

fn advance_timer(time: Res<Time>, mut tick: ResMut<Tick>) {
    let scaled_delta = time.delta().mul_f64(tick.mult);

    tick.timer.tick(scaled_delta);
    tick.mult += FALL_SPEED_UP;
}

fn apply_gravity(tick: ResMut<Tick>, mut query: Query<&mut Transform, With<CurrentShapeTag>>) {
    if tick.timer.just_finished() {
        let mut transform = query.single_mut().unwrap();
        transform.translation.y -= TILE_SIZE;
    };
}

fn bounds(mut query: Query<&mut Transform, With<CurrentShapeTag>>) {
    let mut transform = query.single_mut().unwrap();

    if transform.translation.y < -(TILE_SIZE * ROWS as f32) {
        transform.translation.y = 0.0;
    }
}

#[derive(Component)]
struct CurrentShapeTag;

#[derive(Resource)]
struct CurrentShape(Box<dyn Shape + Send + Sync>);

#[derive(Resource)]
struct Tick {
    timer: Timer,
    mult: f64,
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

// #[derive(Resource)]
// struct Matrix {
//     elements: Vec<Vec<u8>>,
// }

// const MIN_WIDTH: usize = 7;
// const MIN_HEIGHT: usize = 10;

// impl Matrix {
//     fn try_new(width: usize, height: usize) -> Option<Self> {
//         if width < MIN_WIDTH {
//             return None;
//         }
//
//         if height < MIN_HEIGHT {
//             return None;
//         };
//
//         Some(Matrix {
//             elements: vec![vec![0u8; width]; height],
//         })
//     }
//
//     fn height(&self) -> usize {
//         self.elements.len()
//     }
//
//     fn width(&self) -> usize {
//         self.elements.first().unwrap().len()
//     }
//
//     fn set(&mut self, col: usize, row: usize) {
//         self.elements[col][row] = 1;
//     }
//     fn clear(&mut self, col: usize, row: usize) {
//         self.elements[col][row] = 0;
//     }
// }

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
