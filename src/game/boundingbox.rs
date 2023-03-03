

pub struct BoundingBox {
    pub x1: f32,
    pub z1: f32,
    pub x2: f32,
    pub z2: f32,
}


impl BoundingBox {

    pub fn new(x: f32, z: f32, w: f32, h: f32) -> Self {
        Self {
            x1: x - (w / 2.0),
            z1: z - (h / 2.0),
            x2: x + (w / 2.0),
            z2: z + (h / 2.0),
        }
    }

    pub fn update(&mut self, x: f32, z: f32, w: f32, h: f32) {
        self.x1 = x - (w / 2.0);
        self.z1 = z - (h / 2.0);
        self.x2 = x + (w / 2.0);
        self.z2 = z + (h / 2.0);
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
