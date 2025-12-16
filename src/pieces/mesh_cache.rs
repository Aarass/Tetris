use bevy::{asset::Handle, mesh::Mesh};

pub struct MeshCache {
    handles: Vec<Handle<Mesh>>,
}

impl MeshCache {
    pub fn new() -> Self {
        MeshCache {
            handles: Vec::new(),
        }
    }

    pub fn add(&mut self, handle: Handle<Mesh>) {
        self.handles.push(handle);
    }

    pub fn get(&self, index: usize) -> Option<&Handle<Mesh>> {
        self.handles.get(index)
    }
}
