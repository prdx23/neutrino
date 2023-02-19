use core::cell::Cell;

use crate::math::{ Vec3, Matrix4 };
use crate::utils::{ Arena, MemoryBuffer };



pub struct Node<const N: usize> {
    pub children: Arena<usize, N>,
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Vec3,
    meta: bool,
    matrix: Cell<Matrix4>,
}


impl<const N: usize> Default for Node<N> {
    fn default() -> Self {
        Self {
            children: Arena::empty(),
            position: Vec3::zero(),
            scale: Vec3::new(1.0, 1.0, 1.0),
            rotation: Vec3::zero(),
            meta: false,
            matrix: Cell::new(Matrix4::identity()),
        }
    }
}


impl<const N: usize> Node<N> {

    pub fn new() -> Self {
        Default::default()
    }

    pub fn has_meta(&self) -> bool {
        self.meta
    }
}



pub type Tree<const M: usize, const N: usize> = Arena<Node<N>, M>;


impl<const M: usize, const N: usize> Tree<M, N> {

    pub fn root(&mut self) -> usize {
        if self.len() == 0 { self.add(Node::new()); }
        0
    }

    pub fn add_object(&mut self, parent: usize, meta: Option<&str>) -> usize {
        let id = self.add(Node::new());

        if let Some(meta) = meta {
            self[id].meta = true;
            unsafe { crate::add_object(id, meta.as_ptr(), meta.len()); }
        }

        self[parent].children.add(id);

        id
    }

    pub fn update_matrices(
        &self, world_matrix: Matrix4, buffer: &mut MemoryBuffer
    ) {
        self.update_world_matrix(0, world_matrix);
        self.add_matrices_to_buffer(buffer);
    }

    fn update_world_matrix(&self, node: usize, mut matrix: Matrix4) {
        matrix.translate(self[node].position);
        matrix.rotate(self[node].rotation);
        matrix.scale(self[node].scale);

        self[node].matrix.replace(matrix);

        for id in self[node].children.slice() {
            self.update_world_matrix(*id, matrix);
        }
    }

    fn add_matrices_to_buffer(&self, buffer: &mut MemoryBuffer) {
        self.slice()
            .iter()
            .enumerate()
            .filter(|(_, x)| x.has_meta())
            .for_each(|(i, _)| {
                // utils::console_log(format!("id {}", i).as_str());
                buffer.add_f32(i as f32);
                buffer.add_f32(16.0);
                buffer.add_f32(0.0);
                buffer.add_f32(0.0);
                buffer.add_matrix(&self[i].matrix.get());
        });
    }

}
