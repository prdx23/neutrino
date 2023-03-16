use core::cell::Cell;

use crate::math::{ Matrix4 };
use crate::engine::{ Arena, MemoryBuffer };
use crate::engine::entity::{ Entity, EntityBehavior };



const NOBJECTS: usize = 20;
const NCHILDREN: usize = 20;



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
    children: Arena<NodeID, NCHILDREN>,
}


impl Default for Node {
    fn default() -> Self {
        Self {
            entity: Cell::new(Entity::default()),
            children: Arena::empty(),
        }
    }
}


impl Node {
    fn new(entity: Entity) -> Self {
        Self {
            entity: Cell::new(entity),
            children: Arena::empty()
        }
    }
}


// --------------------------------------------------------


pub struct Scenegraph {
    nodes: Arena<Node, NOBJECTS>,
}


impl Scenegraph {

    pub fn empty() -> Self {
        Self {
            nodes: Arena::empty(),
        }
    }


    pub fn root(&mut self) -> NodeID {
        if self.nodes.len() == 0 {
            self.nodes.add(Node::default());
        }
        NodeID::from(0)
    }


    pub fn add_entity(&mut self, parent: NodeID, entity: Entity) -> NodeID {

        let meta = entity.shader_metadata();
        let id = NodeID::from(self.nodes.add(Node::new(entity)));
        self.nodes[parent.0].children.add(id);

        if let Some(m) = meta {
            unsafe { crate::js_add_object(id.0, m.as_ptr(), m.len()) }
        }

        id
    }


    // pub fn load(&mut self, id: NodeID) -> Entity {
    //     self.loaded += 1;
    //     self.nodes[id.0].entity.take()
    // }

    // pub fn store(&mut self, id: NodeID, entity: Entity) {
    //     self.loaded -= 1;
    //     self.nodes[id.0].entity.replace(entity);
    // }

    pub fn with<T, F>(&self, id: NodeID, mut func: F)
    where
        F: FnMut(T) -> T,
        T: From<Entity> + Into<Entity>
    {
        let mut entity = self.nodes[id.0].entity.take();
        entity = func(entity.into()).into();
        self.nodes[id.0].entity.replace(entity);
    }


    // pub fn recursive_update(
    //     &self, id: NodeID, dt: f32,
    //     m: Matrix4, pm: Matrix4, b: &mut MemoryBuffer
    // ) {
    //     if self.loaded > 0 { panic!("not all loaded Node{{}} returned!"); }
    //     self.recursive_update_internal(id, dt, m, pm, b);
    // }

    pub fn recursive_update(
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

        for id in node.children.slice() {
            self.recursive_update(*id, dt, matrix, projection_matrix, buffer);
        }
    }

}
