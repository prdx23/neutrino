
use crate::math::Matrix4;
use crate::engine::Arena;


const BUFFER_SIZE: usize = 400;


#[allow(dead_code)]
#[link_section = "BUFFER_SIZE"]
pub static BSIZE: u32 = BUFFER_SIZE as u32;


pub type MemoryBuffer = Arena<f32, BUFFER_SIZE>;


impl MemoryBuffer {

    pub fn buffer_reset(&mut self) {
        self.reset();
        self.add(0.0);
    }

    pub fn buffer_as_ptr(&mut self) -> *const f32 {
        self[0.into()] = self.len() as f32;
        self.as_ptr()
    }

    pub fn add_float(&mut self, id: f32, ublock: f32, uvar: f32, value: f32) {
        self.add(id);
        self.add(1.0);
        self.add(ublock);
        self.add(uvar);
        self.add(value);
    }

    pub fn add_matrix(
        &mut self, id: f32, ublock: f32, uvar: f32, matrix: &Matrix4
    ) {
        self.add(id);
        self.add(16.0);
        self.add(ublock);
        self.add(uvar);
        for row in matrix.matrix.iter() {
            for value in row.iter() {
                self.add(*value);
            }
        }
    }

}
