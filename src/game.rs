

pub mod object3d;
pub use object3d::Object3d;

pub mod camera;
pub use camera::Camera;

use crate::utils::MemoryBuffer;

pub struct Game {
    pub objects: Vec<Object3d>,
    pub buffer: MemoryBuffer,
    pub camera: Camera,
}
