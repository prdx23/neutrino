
use crate::math::Vec3;


// #[derive(Clone, Copy)]
pub struct BoundingBox {
    pub w: f32,
    pub h: f32,
    pub x1: f32,
    pub z1: f32,
    pub x2: f32,
    pub z2: f32,
}


impl BoundingBox {


    pub fn new(w: f32, h: f32) -> Self {
        Self {
            w, h, x1: 0.0, z1: 0.0, x2: 0.0, z2: 0.0,
        }
    }


    pub fn update(&mut self, position: Vec3) {
        self.x1 = position.x - (self.w / 2.0);
        self.z1 = position.z - (self.h / 2.0);
        self.x2 = position.x + (self.w / 2.0);
        self.z2 = position.z + (self.h / 2.0);
    }


    pub fn collide(&self, other: &Self) -> bool {
        (
            (self.x1 >= other.x1 && self.x1 <= other.x2) ||
            (self.x2 >= other.x1 && self.x2 <= other.x2)
        ) && (
            (self.z1 >= other.z1 && self.z1 <= other.z2) ||
            (self.z2 >= other.z1 && self.z2 <= other.z2)
        )
    }

}
