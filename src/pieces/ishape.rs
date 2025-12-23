use bevy::{
    asset::{Assets, Handle, RenderAssetUsages},
    ecs::system::ResMut,
    mesh::{Indices, Mesh, PrimitiveTopology},
};

use crate::{
    consts::TILE_SIZE,
    pieces::{Piece, Table},
};

const I1: Table = [[1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
const I0: Table = [[1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0]];
const TABLES: [Table; 2] = [I0, I1];

pub struct IShape {
    meshes: [Handle<Mesh>; 2],
    i: usize,
}

impl IShape {
    pub fn new(meshes: &mut ResMut<Assets<Mesh>>) -> Self {
        IShape {
            meshes: [
                meshes.add(
                    Mesh::new(
                        PrimitiveTopology::TriangleStrip,
                        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
                    )
                    .with_inserted_attribute(
                        Mesh::ATTRIBUTE_POSITION,
                        vec![
                            [0.0, 0.0, 0.0],
                            [0.0, -4.0 * TILE_SIZE, 0.0],
                            [1.0 * TILE_SIZE, 0.0, 0.0],
                            [1.0 * TILE_SIZE, -4.0 * TILE_SIZE, 0.0],
                        ],
                    )
                    .with_inserted_indices(Indices::U32(vec![0, 1, 2, 3])),
                ),
                meshes.add(
                    Mesh::new(
                        PrimitiveTopology::TriangleStrip,
                        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
                    )
                    .with_inserted_attribute(
                        Mesh::ATTRIBUTE_POSITION,
                        vec![
                            [0.0, 0.0, 0.0],
                            [0.0, -1.0 * TILE_SIZE, 0.0],
                            [4.0 * TILE_SIZE, 0.0, 0.0],
                            [4.0 * TILE_SIZE, -1.0 * TILE_SIZE, 0.0],
                        ],
                    )
                    .with_inserted_indices(Indices::U32(vec![0, 1, 2, 3])),
                ),
            ],
            i: 0,
        }
    }
}

impl Piece for IShape {
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
