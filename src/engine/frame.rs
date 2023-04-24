
use crate::math::{ Matrix4 };
use crate::engine::{ MemoryBuffer };



pub enum Key {
    W = 0,
    A,
    S,
    D,
    Q,
    E,
    Space,
}



pub struct Frame {
    pub t: f32,
    pub dt: f32,
    pub keys: u8,
    pub projection_matrix: Matrix4,
    pub buffer: MemoryBuffer,
}


impl Frame {

    pub fn new() -> Self {
        Self {
            t: 0.0, dt: 0.0, keys: 0,
            projection_matrix: Matrix4::identity(),
            buffer: MemoryBuffer::empty(),
        }
    }

    pub fn update(&mut self, t: f32, dt: f32, keys: u8, matrix: Matrix4) {
        self.buffer.buffer_reset();
        self.t = t;
        self.dt = dt;
        self.keys = keys;
        self.projection_matrix = matrix;
    }

    pub fn pressed(&self, key: Key) -> bool {
        self.keys & (1 << key as u8) > 0
    }

    pub fn add_view_matrix(&mut self, id: usize, mut matrix: Matrix4) {
        matrix = self.projection_matrix * matrix;
        self.buffer.add_matrix(id, 0.0, 0.0, &matrix);
    }

}
