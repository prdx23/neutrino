
use crate::Matrix4;


const BUFFER_SIZE: usize = 300;


#[allow(dead_code)]
#[link_section = "BUFFER_SIZE"]
pub static BSIZE: u32 = BUFFER_SIZE as u32;


pub struct MemoryBuffer {
    current: usize,
    buffer: [f32; BUFFER_SIZE],
}


impl MemoryBuffer {

    pub fn empty() -> Self {
        Self {
            current: 1,
            buffer: [0.0; BUFFER_SIZE],
        }
    }

    pub fn reset(&mut self) {
        self.current = 1;
        self.buffer[0] = 0.0;
    }


    pub fn as_ptr(&mut self) -> *const f32 {
        self.buffer[0] = self.current as f32;
        self.buffer.as_ptr()
    }


    pub fn add_f32(&mut self, value: f32) {
        self.buffer[self.current] = value;
        self.current += 1;
    }


    pub fn add_matrix(&mut self, matrix: &Matrix4) {
        for row in matrix.matrix.iter() {
            for value in row.iter() {
                self.buffer[self.current] = *value;
                self.current += 1;
            }
        }
    }

}
