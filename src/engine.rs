

pub mod object3d;
pub use object3d::Object3d;

pub mod camera;
pub use camera::Camera;

use crate::utils::{MemoryBuffer, Arena};


pub struct Engine {
    pub objects: Arena<Object3d, 10>,
    pub buffer: MemoryBuffer,
    pub camera: Camera,
}


impl Engine {

    pub fn add_object(&mut self, obj: Object3d, meta: &str) -> usize {
        let id = self.objects.add(obj);
        unsafe { crate::add_object(id, meta.as_ptr(), meta.len()); }
        id
    }

}
