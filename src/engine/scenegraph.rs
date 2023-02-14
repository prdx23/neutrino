use core::cell::Cell;

use crate::math::{ Vec3, Matrix4 };
use crate::utils::Arena;



pub struct Node<const N: usize> {
    pub children: Arena<usize, N>,
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Vec3,
    matrix: Cell<Matrix4>,
}


impl<const N: usize> Default for Node<N> {
    fn default() -> Self {
        Self {
            children: Arena::empty(),
            position: Vec3::zero(),
            scale: Vec3::new(1.0, 1.0, 1.0),
            rotation: Vec3::zero(),
            matrix: Cell::new(Matrix4::identity()),
        }
    }
}


impl<const N: usize> Node<N> {

    pub fn new() -> Self {
        Default::default()
    }

    pub fn matrix(&self) -> Matrix4 {
        self.matrix.get()
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
            unsafe { crate::add_object(id, meta.as_ptr(), meta.len()); }
        }

        self[parent].children.add(id);

        id
    }

    pub fn update_matrices(&self, world_matrix: Matrix4) {
        self.update_world_matrix(0, world_matrix);
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
}
