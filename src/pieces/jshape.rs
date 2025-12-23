use bevy::{
    asset::{Assets, Handle, RenderAssetUsages},
    ecs::system::ResMut,
    mesh::{Indices, Mesh, PrimitiveTopology},
};

use crate::consts::TILE_SIZE;
use crate::pieces::{Piece, Table};

const J0: Table = [
    [0, 1, 0, 0], //
    [0, 1, 0, 0], //
    [1, 1, 0, 0], //
    [0, 0, 0, 0], //
];

const J1: Table = [
    [1, 1, 1, 0], //
    [0, 0, 1, 0], //
    [0, 0, 0, 0], //
    [0, 0, 0, 0], //
];

const J2: Table = [
    [1, 1, 0, 0], //
    [1, 0, 0, 0], //
    [1, 0, 0, 0], //
    [0, 0, 0, 0], //
];

const J3: Table = [
    [1, 0, 0, 0], //
    [1, 1, 1, 0], //
    [0, 0, 0, 0], //
    [0, 0, 0, 0], //
];

const TABLES: [Table; 4] = [J0, J1, J2, J3];

pub struct JShape {
    meshes: [Handle<Mesh>; 4], // TODO
    i: usize,
}

impl JShape {
    pub fn new(meshes: &mut ResMut<Assets<Mesh>>) -> Self {
        JShape {
            meshes: [
                meshes.add(
                    Mesh::new(
                        PrimitiveTopology::TriangleList,
                        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
                    )
                    .with_inserted_attribute(
                        Mesh::ATTRIBUTE_POSITION,
                        vec![
                            [1.0 * TILE_SIZE, 0.0, 0.0],              // 0
                            [1.0 * TILE_SIZE, -3.0 * TILE_SIZE, 0.0], // 1
                            [2.0 * TILE_SIZE, 0.0, 0.0],              // 2
                            [2.0 * TILE_SIZE, -3.0 * TILE_SIZE, 0.0], // 3
                            [1.0 * TILE_SIZE, -2.0 * TILE_SIZE, 0.0], // 4
                            [0.0, -2.0 * TILE_SIZE, 0.0],             // 5
                            [0.0, -3.0 * TILE_SIZE, 0.0],             // 6
                        ],
                    )
                    .with_inserted_indices(Indices::U32(vec![
                        0, 1, 2, //
                        2, 1, 3, //
                        1, 4, 5, //
                        5, 6, 1, //
                    ])),
                ),
                meshes.add(
                    Mesh::new(
                        PrimitiveTopology::TriangleList,
                        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
                    )
                    .with_inserted_attribute(
                        Mesh::ATTRIBUTE_POSITION,
                        vec![
                            [0.0, 0.0, 0.0],                          // 0
                            [0.0, -1.0 * TILE_SIZE, 0.0],             // 1
                            [3.0 * TILE_SIZE, 0.0, 0.0],              // 2
                            [3.0 * TILE_SIZE, -1.0 * TILE_SIZE, 0.0], // 3
                            [2.0 * TILE_SIZE, -1.0 * TILE_SIZE, 0.0], // 4
                            [2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 0.0], // 5
                            [3.0 * TILE_SIZE, -2.0 * TILE_SIZE, 0.0], // 6
                        ],
                    )
                    .with_inserted_indices(Indices::U32(vec![
                        0, 1, 2, //
                        2, 1, 3, //
                        3, 4, 5, //
                        3, 5, 6, //
                    ])),
                ),
                meshes.add(
                    Mesh::new(
                        PrimitiveTopology::TriangleList,
                        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
                    )
                    .with_inserted_attribute(
                        Mesh::ATTRIBUTE_POSITION,
                        vec![
                            [0.0, 0.0, 0.0],                          // 0
                            [0.0, -3.0 * TILE_SIZE, 0.0],             // 1
                            [1.0 * TILE_SIZE, 0.0, 0.0],              // 2
                            [1.0 * TILE_SIZE, -3.0 * TILE_SIZE, 0.0], // 3
                            [1.0 * TILE_SIZE, -1.0 * TILE_SIZE, 0.0], // 4
                            [2.0 * TILE_SIZE, 0.0, 0.0],              // 5
                            [2.0 * TILE_SIZE, -1.0 * TILE_SIZE, 0.0], // 6
                        ],
                    )
                    .with_inserted_indices(Indices::U32(vec![
                        0, 1, 2, //
                        2, 1, 3, //
                        2, 4, 5, //
                        5, 4, 6, //
                    ])),
                ),
                meshes.add(
                    Mesh::new(
                        PrimitiveTopology::TriangleList,
                        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
                    )
                    .with_inserted_attribute(
                        Mesh::ATTRIBUTE_POSITION,
                        vec![
                            [0.0, 0.0, 0.0],                          // 0
                            [0.0, -1.0 * TILE_SIZE, 0.0],             // 1
                            [1.0 * TILE_SIZE, 0.0, 0.0],              // 2
                            [1.0 * TILE_SIZE, -1.0 * TILE_SIZE, 0.0], // 3
                            [0.0, -2.0 * TILE_SIZE, 0.0],             // 4
                            [3.0 * TILE_SIZE, -1.0 * TILE_SIZE, 0.0], // 5
                            [3.0 * TILE_SIZE, -2.0 * TILE_SIZE, 0.0], // 6
                        ],
                    )
                    .with_inserted_indices(Indices::U32(vec![
                        0, 1, 2, //
                        2, 1, 3, //
                        1, 4, 5, //
                        5, 4, 6, //
                    ])),
                ),
            ],
            i: 0,
        }
    }
}

impl Piece for JShape {
    fn rotate_cw(&mut self) {
        if self.i == 0 {
            self.i = TABLES.len() - 1;
        } else {
            self.i = self.i - 1;
        }
    }

    fn rotate_ccw(&mut self) {
        self.i = (self.i + 1) % TABLES.len();
    }

    fn get_table(&self) -> &Table {
        &TABLES[self.i]
    }

    fn get_mesh(&self) -> &Handle<Mesh> {
        &self.meshes[self.i]
    }
}
