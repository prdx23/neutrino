

use crate::math::{ Vec3, Matrix4 };
use crate::physics::{ RigidBody };
use crate::engine::entity::EntityBehavior;


pub struct Thruster {
    position: Vec3,
    direction: Vec3,
    thrust: f32,
    firing: bool,
    matrix: Matrix4,
}


impl Thruster {

    const META: &'static str = r#"{
        "shader": "test",
        "count": 6,
        "attributes": {
            "a_position": "quad"
        },
        "uniforms": {
            "objectData": ["u_matrix"]
        }
    }"#;


    const ORIGIN: Vec3 = Vec3::zero();

    pub const fn new(position: Vec3, direction: Vec3, thrust: f32) -> Self {
        Self {
            position, direction, thrust,
            firing: false,
            matrix: Matrix4::identity(),
        }
    }

    pub fn fire(&mut self, rigidbody: &mut RigidBody) {
        let center = self.matrix * Self::ORIGIN;
        let distance = (self.matrix * self.position) - center;
        let transformed_force = (self.matrix * self.direction) - center;
        let force = transformed_force.unit() * self.thrust;
        rigidbody.apply_force_and_torque(force, distance);
        self.firing = true
    }

}

impl EntityBehavior for Thruster {

    fn update_matrix(&mut self, _: f32, mut matrix: Matrix4) -> Matrix4 {
        self.matrix = matrix.clone();
        if self.firing {
            matrix.translate(
                self.position + (self.direction * -1.0 * self.thrust * 4.0)
            );
            self.firing = false;
        }
        matrix
    }

    fn shader_metadata(&self) -> Option<&'static str> {
        Some(Self::META)
    }

}
