use bevy::{
    asset::{Assets, Handle, RenderAssetUsages},
    ecs::system::ResMut,
    mesh::{Indices, Mesh, PrimitiveTopology},
};

use crate::{
    consts::TILE_SIZE,
    pieces::{Shape, Table},
};

const O0: Table = [[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];

pub struct OShape {
    mesh: Handle<Mesh>,
}

impl OShape {
    pub fn new(meshes: &mut ResMut<Assets<Mesh>>) -> Self {
        OShape {
            mesh: meshes.add(
                Mesh::new(
                    PrimitiveTopology::TriangleStrip,
                    RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
                )
                .with_inserted_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    vec![
                        [0.0, 0.0, 0.0],
                        [0.0, -2.0 * TILE_SIZE, 0.0],
                        [2.0 * TILE_SIZE, 0.0, 0.0],
                        [2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 0.0],
                    ],
                )
                .with_inserted_indices(Indices::U32(vec![0, 1, 2, 3])),
            ),
        }
    }
}

impl Shape for OShape {
    fn rotate_cw(&mut self) {}
    fn rotate_ccw(&mut self) {}

    fn get_table(&self) -> &Table {
        &O0
    }

    fn get_mesh(&self) -> &Handle<Mesh> {
        &self.mesh
    }
}
