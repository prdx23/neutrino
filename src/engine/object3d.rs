
use crate::{ Vec3, Matrix4 };

#[derive(Clone, Copy)]
pub struct Object3d {
    pub id: f32,
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Vec3,
}

impl Object3d {

    pub fn empty() -> Self {
        Self {
            id: 0.0,
            position: Vec3::zero(),
            scale: Vec3::zero(),
            rotation: Vec3::zero(),
        }
    }

    pub fn get_matrix(&self, view_projection_matrix: Matrix4) -> Matrix4 {
        let mut matrix = Matrix4::identity();
        matrix.rotate(self.rotation);
        matrix.scale(self.scale);
        matrix.translate(self.position);
        view_projection_matrix * matrix
    }

}
