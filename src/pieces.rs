mod ishape;
mod lshape;
mod oshape;

pub mod mesh_cache;

use bevy::{asset::Handle, mesh::Mesh};

pub use ishape::IShape;
pub use lshape::LShape;
pub use oshape::OShape;

pub trait Shape {
    fn rotate_cw(&mut self);
    fn rotate_ccw(&mut self);
    fn get_table(&self) -> &Table;
    fn get_mesh(&self) -> &Handle<Mesh>;
}

pub type Table = [[u8; 4]; 4];
