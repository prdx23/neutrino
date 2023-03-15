use core::cell::Cell;

use crate::math::{ Vec3, Matrix4 };
use crate::engine::{ Arena, ArenaID, MemoryBuffer };
use crate::physics::{ RigidBody, Aabb };



pub struct Node<const N: usize> {
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Vec3,
    pub rigidbody: RigidBody,
    pub aabb: Aabb,
    meta: Option<&'static str>,
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
            aabb: Aabb::default(),
            meta: None,
            matrix: Matrix4::identity(),
            children: Arena::empty(),
        }
    }
}


impl<const N: usize> Node<N> {

    pub fn new(meta: Option<&'static str>) -> Self {
        let mut node = Self::default();
        node.meta = meta;
        node
    }


    pub fn update_matrix(&mut self, dt: f32, mut matrix: Matrix4) -> Matrix4 {

        (self.position, self.rotation) = self.rigidbody.update_physics(
            self.position, self.rotation, dt
        );

        self.aabb.update(self.position);

        matrix.translate(self.position);
        matrix.rotate(self.rotation);
        matrix.scale(self.scale);
        self.matrix = matrix.clone();

        matrix
    }

}


// --------------------------------------------------------



pub struct Scenegraph<const M: usize, const N: usize> {
    nodes: Arena<Cell<Node<N>>, M>,
    loaded: u16,
}


impl<const M: usize, const N: usize> Scenegraph<M, N> {

    pub fn empty() -> Self {
        Self {
            nodes: Arena::empty(),
            loaded: 0,
        }
    }


    pub fn root(&mut self) -> ArenaID {
        if self.nodes.len() == 0 {
            self.nodes.add(Node::default().into());
        }
        ArenaID::from(0)
    }


    pub fn add_object(&mut self, parent: ArenaID, node: Node<N>) -> ArenaID {
        let id = match node.meta {
            Some(m) => {
                let id = self.nodes.add(node.into());
                unsafe { crate::js_add_object(id.into(), m.as_ptr(), m.len()) }
                id
            },
            None => self.nodes.add(node.into())
        };
        self.nodes[parent].get_mut().children.add(id);
        id
    }


    pub fn load(&mut self, id: ArenaID) -> Node<N> {
        self.loaded += 1;
        self.nodes[id].take()
    }

    pub fn store(&mut self, id: ArenaID, node: Node<N>) {
        self.loaded -= 1;
        self.nodes[id].replace(node);
    }

    pub fn load_mut_ref(&mut self, id: ArenaID) -> &mut Node<N> {
        self.nodes[id].get_mut()
    }


    pub fn recursive_update(
        &mut self, id: ArenaID, dt: f32,
        m: Matrix4, pm: Matrix4, b: &mut MemoryBuffer
    ) {
        if self.loaded > 0 { panic!("not all loaded Node{{}} returned!"); }
        self.recursive_update_internal(id, dt, m, pm, b);
    }

    fn recursive_update_internal(
        &mut self, nodeid: ArenaID, dt: f32,
        mut matrix: Matrix4, projection_matrix: Matrix4,
        buffer: &mut MemoryBuffer
    ) {

        let node = self.nodes[nodeid].get_mut();
        matrix = node.update_matrix(dt, matrix);

        if node.meta.is_some() {
            let view_matrix = projection_matrix * matrix;
            buffer.add_matrix(nodeid.into(), 0.0, 0.0, &view_matrix);
        }

        for id in node.children.clone().slice() {
            self.recursive_update(*id, dt, matrix, projection_matrix, buffer);
        }
    }

}
