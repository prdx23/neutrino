use core::cell::Cell;

use crate::math::{ Matrix4 };
use crate::engine::{ Arena, MemoryBuffer };
use crate::engine::entity::{ Entity, EntityBehavior };



const NOBJECTS: usize = 100;
const NCHILDREN: usize = 10;



#[derive(Default, Clone, Copy)]
pub struct NodeID(usize);

impl From<usize> for NodeID {
    fn from(value: usize) -> Self { Self(value) }
}

impl From<NodeID> for usize {
    fn from(value: NodeID) -> usize { value.0 }
}

impl From<NodeID> for f32 {
    fn from(value: NodeID) -> f32 { value.0 as f32 }
}


// --------------------------------------------------------


struct Node {
    entity: Cell<Entity>,
    // children: Arena<NodeID, NCHILDREN>,
    children: Vec<NodeID>,
}


impl Default for Node {
    fn default() -> Self {
        Self {
            entity: Cell::new(Entity::default()),
            // children: Arena::empty(),
            children: Vec::with_capacity(NCHILDREN),
        }
    }
}


impl Node {
    fn new(entity: Entity) -> Self {
        Self {
            entity: Cell::new(entity),
            // children: Arena::empty()
            children: Vec::with_capacity(NCHILDREN),
        }
    }
}


// --------------------------------------------------------


pub struct Scenegraph {
    // nodes: Arena<Node, NOBJECTS>,
    nodes: Vec<Node>,
    loaded: u16,
}


impl Scenegraph {

    pub fn empty() -> Self {
        Self {
            // nodes: Arena::empty(),
            nodes: Vec::with_capacity(NOBJECTS),
            loaded: 0,
        }
    }


    pub fn root(&mut self) -> NodeID {
        if self.nodes.len() == 0 {
            // self.nodes.add(Node::default());
            self.nodes.push(Node::default());
        }
        NodeID::from(0)
    }


    pub fn add_entity(&mut self, parent: NodeID, entity: Entity) -> NodeID {

        let meta = entity.shader_metadata();
        // let id = NodeID::from(self.nodes.add(Node::new(entity)));
        // self.nodes[parent.0].children.add(id);
        self.nodes.push(Node::new(entity));
        let id = NodeID::from(self.nodes.len() - 1);
        self.nodes[parent.0].children.push(id);

        if let Some(m) = meta {
            unsafe { crate::js_add_object(id.0, m.as_ptr(), m.len()) }
        }

        id
    }


    pub fn load(&mut self, id: NodeID) -> Entity {
        self.loaded += 1;
        self.nodes[id.0].entity.take()
    }

    pub fn store(&mut self, id: NodeID, entity: Entity) {
        self.loaded -= 1;
        self.nodes[id.0].entity.replace(entity);
    }

    pub fn with<T, F>(&self, id: NodeID, mut func: F)
    where
        F: FnMut(T) -> T,
        T: From<Entity> + Into<Entity>
    {
        let mut entity = self.nodes[id.0].entity.take();
        entity = func(entity.into()).into();
        self.nodes[id.0].entity.replace(entity);
    }


    pub fn recursive_update(
        &self, id: NodeID, dt: f32,
        m: Matrix4, pm: Matrix4, b: &mut MemoryBuffer
    ) {
        if self.loaded > 0 { panic!("not all loaded Node{{}} returned!"); }
        self.recursive_update_internal(id, dt, m, pm, b);
    }

    fn recursive_update_internal(
        &self, nodeid: NodeID, dt: f32,
        mut matrix: Matrix4, projection_matrix: Matrix4,
        buffer: &mut MemoryBuffer
    ) {

        let node = &self.nodes[nodeid.0];
        let mut entity = node.entity.take();

        matrix = entity.update_matrix(dt, matrix);

        if entity.shader_metadata().is_some() {
            let view_matrix = projection_matrix * matrix;
            buffer.add_matrix(f32::from(nodeid), 0.0, 0.0, &view_matrix);
        }

        node.entity.replace(entity);

        // for id in node.children.slice() {
        for id in node.children.iter() {
            self.recursive_update(*id, dt, matrix, projection_matrix, buffer);
        }
    }

}
