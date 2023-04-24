
pub mod camera;
pub use camera::Camera;

pub mod arena;
pub use arena::{ Arena, ArenaID };

pub mod memory_buffer;
pub use memory_buffer::MemoryBuffer;

pub mod entity;

pub mod frame;
pub use frame::{Key, Frame};


use crate::game::{Game};


pub struct Engine {
    pub camera: Camera,
    pub frame: Frame,
    pub game: Game,
}
