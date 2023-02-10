
use crate::{ Vec3, Matrix4 };

#[derive(Clone, Copy)]
pub struct Object3d {
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Vec3,
}


impl Default for Object3d {
    fn default() -> Self {
        Self {
            position: Vec3::zero(),
            scale: Vec3::zero(),
            rotation: Vec3::zero(),
        }
    }
}


impl Object3d {

    pub fn get_matrix(&self, view_projection_matrix: Matrix4) -> Matrix4 {
        let mut matrix = Matrix4::identity();
        matrix.rotate(self.rotation);
        matrix.scale(self.scale);
        matrix.translate(self.position);
        view_projection_matrix * matrix
    }

}
