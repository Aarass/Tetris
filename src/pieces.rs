mod ishape;
mod jshape;
mod lshape;
mod oshape;
mod sshape;
mod tshape;
mod zshape;

pub mod mesh_cache;

use bevy::{
    asset::Handle,
    math::{Vec2, Vec3Swizzles},
    mesh::Mesh,
    transform::components::Transform,
};

pub use ishape::IShape;
pub use jshape::JShape;
pub use lshape::LShape;
pub use oshape::OShape;
pub use sshape::SShape;
pub use tshape::TShape;
pub use zshape::ZShape;

use crate::consts::TILE_SIZE;

pub type PieceType = Box<dyn Piece + Send + Sync>;

pub trait Piece {
    fn rotate_cw(&mut self);
    fn rotate_ccw(&mut self);
    fn get_table(&self) -> &Table;
    fn get_mesh(&self) -> &Handle<Mesh>;
}

pub fn get_piece_indicies(transform: &Transform) -> PieceIndicies {
    (transform.translation.xy().abs() / TILE_SIZE).into()
}

pub type Table = [[u8; 4]; 4];

#[derive(Debug)]
pub struct PieceIndicies {
    pub i: usize,
    pub j: usize,
}

impl Into<PieceIndicies> for Vec2 {
    fn into(self) -> PieceIndicies {
        PieceIndicies {
            i: self.x as usize,
            j: self.y as usize,
        }
    }
}
