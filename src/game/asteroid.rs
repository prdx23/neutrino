
use crate::math::{ Vec3, Matrix4 };
// use crate::physics::{ RigidBody };
use crate::engine::{ Frame };
use crate::engine::entity::{ EntityBehavior };
use crate::utils;



pub struct Asteroid {
    id: usize,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
    // rigidbody: RigidBody,
}


impl Default for Asteroid {
    fn default() -> Self {
        Self {
            id: 0,
            position: Vec3::zero(),
            rotation: Vec3::zero(),
            scale: Vec3::new(1.0, 1.0, 1.0),
            // rigidbody: RigidBody::new(1.0, 0.0),
        }
    }
}


impl Asteroid {

    pub fn new() -> Self {
        let mut object = Self::default();
        object.id = utils::webgl_add_entity(r#"{
            "shader": "vertex_color",
            "count": 36,
            "attributes": {
                "a_position": "cube_vertices",
                "a_color": "cube_vertex_colors"
            },
            "uniforms": {
                "objectData": ["u_matrix"]
            }
        }"#);
        object
    }

}


impl EntityBehavior for Asteroid {

    fn render_frame(&mut self, frame: &mut Frame) {
        // self.rigidbody.update_physics(
        //     frame.dt, &mut self.position, &mut self.rotation
        // );
    }

    fn update_uniforms(&mut self, frame: &mut Frame, mut matrix: Matrix4) {
        matrix.translate(self.position);
        matrix.rotate(self.rotation);
        matrix.scale(self.scale);
        frame.add_view_matrix(self.id, matrix);
    }

}
