
use crate::math::{ Vec3, Matrix4 };
use crate::engine::{ Arena, ArenaID, MemoryBuffer };
use crate::physics::{ RigidBody };



pub struct Node<const N: usize> {
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Vec3,
    pub rigidbody: RigidBody,

    meta: bool,
    matrix: Matrix4,
    children: Arena<ArenaID, N>,
}


impl<const N: usize> Default for Node<N> {
    fn default() -> Self {
        Self {
            position: Vec3::zero(),
            scale: Vec3::new(1.0, 1.0, 1.0),
            rotation: Vec3::zero(),
            rigidbody: RigidBody::default(),

            meta: false,
            matrix: Matrix4::identity(),
            children: Arena::empty(),
        }
    }
}


impl<const N: usize> Node<N> {

    pub fn has_meta(&self) -> bool {
        self.meta
    }


    pub fn update_matrix(&mut self, dt: f32, mut matrix: Matrix4) -> Matrix4 {

        (self.position, self.rotation) = self.rigidbody.update_physics(
            self.position, self.rotation, dt
        );

        matrix.translate(self.position);
        matrix.rotate(self.rotation);
        matrix.scale(self.scale);
        self.matrix = matrix.clone();

        matrix
    }

}


// --------------------------------------------------------



pub type Scenegraph<const M: usize, const N: usize> = Arena<Node<N>, M>;


impl<const M: usize, const N: usize> Scenegraph<M, N> {


    pub fn root(&mut self) -> ArenaID {
        if self.len() == 0 { self.add(Node::default()); }
        ArenaID::from(0)
    }


    pub fn add_object(&mut self, parent: ArenaID, meta: Option<&str>) -> ArenaID {
        let id = self.add(Node::default());

        if let Some(meta) = meta {
            self[id].meta = true;
            unsafe { crate::add_object(id.into(), meta.as_ptr(), meta.len()); }
        }

        self[parent].children.add(id);
        id
    }


    pub fn recursive_update(
        &mut self, node: ArenaID, dt: f32,
        mut matrix: Matrix4, projection_matrix: Matrix4,
        buffer: &mut MemoryBuffer
    ) {

        matrix = self[node].update_matrix(dt, matrix);

        if self[node].has_meta() {
            let view_matrix = projection_matrix * matrix;
            let id: usize = node.into();
            buffer.add_matrix(id as f32, 0.0, 0.0, &view_matrix);
        }

        for id in self[node].children.clone().slice() {
            self.recursive_update(*id, dt, matrix, projection_matrix, buffer);
        }
    }

}
