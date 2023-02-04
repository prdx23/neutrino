
use crate::{ Vec3, Matrix4 };

// #[derive(Clone, Copy)]
pub struct Camera {
    pub position: Vec3,
    pub up: Vec3,
    matrix: Matrix4,
    projection_matrix: Matrix4,
}


impl Camera {

    pub fn perspective(
        position: Vec3, fov: f32, aspect: f32, near: f32, far: f32
    ) -> Self {

        let up = Vec3::new(0.0, 1.0, 0.0);
        Self {
            position: position,
            up: up,
            matrix: Matrix4::look_at(position, Vec3::zero(), up),
            projection_matrix: Matrix4::perspective(fov, aspect, near, far),
        }

    }

    pub fn look_at(&mut self, target: Vec3) {
        self.matrix = Matrix4::look_at(self.position, target, self.up);
    }

    pub fn view_projection_matrix(&self) -> Matrix4 {
        self.projection_matrix * self.matrix.inverse()
    }

}
