mod ishape;
mod jshape;
mod lshape;
mod oshape;
mod sshape;
mod tshape;
mod zshape;

pub mod mesh_cache;

use bevy::{asset::Handle, mesh::Mesh, transform::components::Transform};

pub use ishape::IShape;
pub use jshape::JShape;
pub use lshape::LShape;
pub use oshape::OShape;
pub use sshape::SShape;
pub use tshape::TShape;
pub use zshape::ZShape;

use crate::consts::TILE_SIZE;

pub type BoxedPiece = Box<dyn Piece + Send + Sync>;

pub trait Piece {
    fn rotate_cw(&mut self);
    fn rotate_ccw(&mut self);
    fn get_table(&self) -> &Table;
    fn get_mesh(&self) -> &Handle<Mesh>;
}

pub fn get_piece_indicies(transform: &Transform) -> PieceIndicies {
    PieceIndicies {
        i: (-transform.translation.y / TILE_SIZE) as i32,
        j: (transform.translation.x / TILE_SIZE) as i32,
    }
}

pub type Table = [[u8; 4]; 4];

#[derive(Debug)]
pub struct PieceIndicies {
    pub i: i32,
    pub j: i32,
}
