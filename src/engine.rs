
pub mod camera;
pub use camera::Camera;

pub mod scenegraph;
pub use scenegraph::Tree;

use crate::utils::{MemoryBuffer};
use crate::game::{Game};


pub struct Engine {
    pub buffer: MemoryBuffer,
    pub camera: Camera,
    pub game: Game,
}
