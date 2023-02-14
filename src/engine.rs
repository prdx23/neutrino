
pub mod camera;
pub use camera::Camera;

pub mod scenegraph;
pub use scenegraph::Tree;

use crate::utils::{MemoryBuffer};

const NOBJECTS: usize = 9;
const NCHILDREN: usize = 9;


pub struct Engine {
    pub buffer: MemoryBuffer,
    pub camera: Camera,
    pub scenegraph: Tree<NOBJECTS, NCHILDREN>,
}
