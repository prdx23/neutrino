

use crate::math::{ Vec3, Matrix4 };
use crate::physics::{ RigidBody };
use crate::engine::entity::EntityBehavior;


pub struct Ship {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
    pub rigidbody: RigidBody,
    shader_metadata: &'static str,
}


impl Ship {
    pub fn new() -> Self {
        let meta = r#"{
            "shader": "vertex_color",
            "count": 36,
            "attributes": {
                "a_position": "cube_vertices",
                "a_color": "cube_vertex_colors"
            },
            "uniforms": {
                "objectData": ["u_matrix"]
            }
        }"#;
        let mut rb = RigidBody::default();
        rb.enable(100.0, 2.0);
        Self {
            position: Vec3::zero(),
            rotation: Vec3::zero(),
            scale: Vec3::new(5.0, 5.0, 5.0),
            rigidbody: rb,
            shader_metadata: meta,
        }
    }
}


impl EntityBehavior for Ship {

    fn update_matrix(&mut self, dt: f32, mut matrix: Matrix4) -> Matrix4 {
        (self.position, self.rotation) = self.rigidbody.update_physics(
            self.position, self.rotation, dt
        );
        matrix.translate(self.position);
        matrix.rotate(self.rotation);
        matrix.scale(self.scale);
        matrix
    }

    fn shader_metadata(&self) -> Option<&'static str> {
        Some(self.shader_metadata)
    }

}
