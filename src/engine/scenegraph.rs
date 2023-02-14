
use crate::math::{ Vec3, Matrix4 };
use crate::utils::Arena;
use crate::engine::NOBJ;


const NN: usize = 10;


#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub children: Arena<usize, NN>,
    pub matrix: Matrix4,
    pub world_matrix: Matrix4,
}


impl Default for Node {
    fn default() -> Self {
        Self {
            children: Arena::empty(),
            matrix: Matrix4::identity(),
            world_matrix: Matrix4::identity(),
        }
    }
}


impl Node {

    pub fn new() -> Self {
        Default::default()
    }

}




#[derive(Debug)]
pub struct Tree<const N: usize> {
    nodes: Arena<Node, N>,
}


impl<const N: usize> Tree<N> {

    pub fn empty() -> Tree<N> {
        Tree {
            nodes: Arena::empty()
        }
    }

    pub fn new_root(&mut self) -> usize {
        self.nodes.add(Node::new())
    }

    pub fn new_node(&mut self, parent: usize) -> usize {
        let id = self.nodes.add(Node::new());
        self.nodes.get_mut(parent).children.add(id);
        id
    }

    // pub fn node(&self, node: usize) -> &Node {
    //     self.nodes.get(node)
    // }

    pub fn node(&mut self, node: usize) -> &mut Node {
        self.nodes.get_mut(node)
    }

    pub fn update_world_matrix(&mut self, node: usize, mut world_matrix: Matrix4) {
        let matrix = world_matrix * self.node(node).matrix;
        // crate::utils::console_log(format!("update id {} {:?}", node, matrix).as_str());
        // world_matrix *= self.node(node).matrix;
        self.nodes.get_mut(node).world_matrix = matrix;
        for id in self.nodes.get(node).children.clone().iter() {
            self.update_world_matrix(*id, matrix);
        }
    }
}
