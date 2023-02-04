

pub mod object3d;
pub use object3d::Object3d;

pub mod camera;
pub use camera::Camera;

use crate::utils::MemoryBuffer;
use crate::utils::ObjectArray;

pub struct Game {
    pub objects: ObjectArray,
    pub buffer: MemoryBuffer,
    pub camera: Camera,
}
