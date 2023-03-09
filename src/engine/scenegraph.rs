
use crate::math::{ Vec3, Matrix4 };
use crate::utils::{ Arena, MemoryBuffer };
use crate::engine::{ BoundingBox };
use crate::physics::{ RigidBody };



pub struct Node<const N: usize> {
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Vec3,

    pub rigidbody: RigidBody,

    meta: bool,
    matrix: Matrix4,
    bbox: Option<BoundingBox>,

    children: Arena<usize, N>,
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
            bbox: None,
            children: Arena::empty(),
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

    pub fn add_bbox(&mut self, bbox: BoundingBox) {
        self.bbox = Some(bbox);
    }

    pub fn update_matrix(&mut self, mut matrix: Matrix4) -> Matrix4 {
        matrix.translate(self.position);
        matrix.rotate(self.rotation);
        matrix.scale(self.scale);
        self.matrix = matrix.clone();
        matrix
    }

    pub fn update_bbox(&mut self) {
        if let Some(bbox) = self.bbox.as_mut() {
            (*bbox).update(self.position);
        }
    }


    pub fn update_physics(&mut self, dt: f32) {
        if self.rigidbody.is_enabled() {
            (self.position, self.rotation) = self.rigidbody.update_physics(
                self.position, self.rotation, dt
            );
        }
    }

}


// --------------------------------------------------------



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


    pub fn update(
        &mut self, dt: f32, world_matrix: Matrix4, buffer: &mut MemoryBuffer
    ) {
        self.update_node(0, dt, world_matrix);
        self.add_matrices_to_buffer(buffer);
    }


    fn update_node(&mut self, node: usize, dt: f32, mut matrix: Matrix4) {
        self[node].update_physics(dt);
        self[node].update_bbox();

        matrix = self[node].update_matrix(matrix);
        for id in self[node].children.clone().slice() {
            self.update_node(*id, dt, matrix);
        }
    }


    fn add_matrices_to_buffer(&self, buffer: &mut MemoryBuffer) {
        self.slice()
            .iter()
            .enumerate()
            .filter(|(_, x)| x.has_meta())
            .for_each(|(i, _)| {
                // utils::console_log(format!("id {}", i).as_str());
                buffer.add_matrix(i as f32, 0.0, 0.0, &self[i].matrix);
        });
    }


    pub fn collide(&self, node1: usize, node2: usize) -> bool {
        if let Some(bb1) = &self[node1].bbox {
            if let Some(bb2) = &self[node2].bbox {
                return bb1.collide(bb2);
            }
        }
        false
    }

}
