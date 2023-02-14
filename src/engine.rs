

pub mod object3d;
pub use object3d::Object3d;

pub mod camera;
pub use camera::Camera;

pub mod scenegraph;
pub use scenegraph::Tree;

use crate::utils::{MemoryBuffer, Arena};

pub const NOBJ: usize = 9;


pub struct Engine {
    pub tree: Tree<NOBJ>,
    pub buffer: MemoryBuffer,
    pub camera: Camera,
}


impl Engine {


    // pub fn add_object(&mut self, obj: Object3d, meta: &str) -> usize {
    //     let id = self.objects.add(obj);
    //     unsafe { crate::add_object(id, meta.as_ptr(), meta.len()); }
    //     id
    // }

}
