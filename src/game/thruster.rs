
use crate::math::{ Vec3, Matrix4 };
use crate::engine::{ Frame };
use crate::physics::{ RigidBody };
use crate::engine::entity::{ EntityBehavior };
use crate::utils;



pub struct Thruster {
    id: usize,
    position: Vec3,
    direction: Vec3,
    thrust: f32,
    firing: bool,
    matrix: Matrix4,
}



impl Thruster {

    pub fn new(position: Vec3, direction: Vec3, thrust: f32) -> Self {
        Self {
            id: utils::webgl_add_entity(r#"{
                "shader": "test",
                "count": 6,
                "attributes": {
                    "a_position": "quad"
                },
                "uniforms": {
                    "objectData": ["u_matrix"]
                }
            }"#),
            position,
            direction: direction.unit(),
            thrust: thrust * 1000.0,   // in kN
            firing: false,
            matrix: Matrix4::identity(),
        }
    }

    const ORIGIN: Vec3 = Vec3::zero();

    pub fn fire(&mut self, rigidbody: &mut RigidBody, throttle: f32) {
        rigidbody.apply_force_and_torque(
            -self.exhaust_dir() * (self.thrust * throttle),
            self.distance_from_center()
        );
        self.firing = true;
    }

    pub fn exhaust_dir(&self) -> Vec3 {
        let center = self.matrix * Self::ORIGIN;
        ((self.matrix * self.direction) - center).unit()
    }

    pub fn distance_from_center(&self) -> Vec3 {
        let center = self.matrix * Self::ORIGIN;
        (self.matrix * self.position) - center
    }

}

impl EntityBehavior for Thruster {

    fn render_frame(&mut self, _: &mut Frame) {
    }

    fn update_uniforms(&mut self, frame: &mut Frame, mut matrix: Matrix4) {
        self.matrix = matrix.clone();
        if self.firing {
            matrix.translate(
                self.position + (self.direction * 1.0 * 4.0)
            );
            self.firing = false;
        }
        frame.add_view_matrix(self.id, matrix);
    }

}
