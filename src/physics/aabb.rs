
use crate::math::Vec3;


// #[derive(Clone, Copy)]
pub struct Aabb {
    pub w: f32,
    pub h: f32,
    x1: f32,
    z1: f32,
    x2: f32,
    z2: f32,
    pub colliding: bool,
}

impl Default for Aabb {
    fn default() -> Self {
        Self {
            colliding: false,
            w: 0.0, h: 0.0,
            x1: 0.0, z1: 0.0, x2: 0.0, z2: 0.0,
        }
    }
}


impl Aabb {


    pub fn new(w: f32, h: f32) -> Self {
        // self.enabled = true;
        let mut aabb = Self::default();
        aabb.w = w;
        aabb.h = h;
        aabb
    }


    pub fn update(&mut self, position: Vec3) {
        // if !self.enabled { return; }
        self.x1 = position.x - (self.w / 2.0);
        self.z1 = position.z - (self.h / 2.0);
        self.x2 = position.x + (self.w / 2.0);
        self.z2 = position.z + (self.h / 2.0);
        // self.colliding = false;
    }


    pub fn collide(&mut self, other: &mut Self) -> bool {
        // if !self.enabled { return false; }
        if self.x2 < other.x1 { return false; }
        if self.x1 > other.x2 { return false; }
        if self.z2 < other.z1 { return false; }
        if self.z1 > other.z2 { return false; }
        self.colliding = true;
        other.colliding = true;
        true
    }

}
