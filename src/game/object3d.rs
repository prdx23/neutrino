
use crate::math::{ Vec3, Matrix4 };
use crate::engine::{ Frame };
use crate::engine::entity::{ EntityBehavior };
use crate::utils;



pub struct Object3d {
    id: usize,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}


impl Default for Object3d {
    fn default() -> Self {
        Self {
            id: 0,
            position: Vec3::zero(),
            rotation: Vec3::zero(),
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }
}


impl Object3d {
    pub fn new(meta: &'static str) -> Self {
        let mut object = Self::default();
        object.id = utils::webgl_add_entity(meta);
        object
    }
}


impl EntityBehavior for Object3d {

    fn render_frame(&mut self, _: &mut Frame) {
    }

    fn update_uniforms(&mut self, frame: &mut Frame, mut matrix: Matrix4) {
        matrix.translate(self.position);
        matrix.rotate(self.rotation);
        matrix.scale(self.scale);
        frame.add_view_matrix(self.id, matrix);
    }

}
