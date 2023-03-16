
pub mod camera;
pub use camera::Camera;

pub mod arena;
pub use arena::Arena;

pub mod memory_buffer;
pub use memory_buffer::MemoryBuffer;

pub mod scenegraph;
pub mod entity;


use crate::game::{Game};


pub struct Engine {
    pub buffer: MemoryBuffer,
    pub camera: Camera,
    pub game: Game,
}
