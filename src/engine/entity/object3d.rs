

use crate::math::{ Vec3, Matrix4 };
use crate::engine::entity::EntityBehavior;


pub struct Object3d {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
    shader_metadata: Option<&'static str>,
}


impl Object3d {
    pub fn new(meta: Option<&'static str>) -> Self {
        Self {
            position: Vec3::zero(),
            rotation: Vec3::zero(),
            scale: Vec3::new(1.0, 1.0, 1.0),
            shader_metadata: meta,
        }
    }
}


impl EntityBehavior for Object3d {

    fn update_matrix(&mut self, _: f32, mut matrix: Matrix4) -> Matrix4 {
        matrix.translate(self.position);
        matrix.rotate(self.rotation);
        matrix.scale(self.scale);
        matrix
    }

    fn shader_metadata(&self) -> Option<&'static str> {
        self.shader_metadata
    }

}
