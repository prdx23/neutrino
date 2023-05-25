
use crate::math::{ Vec3, Matrix4 };
use crate::physics;
use crate::engine::{ Frame };
use crate::engine::entity::{ EntityBehavior };
use crate::utils;



pub struct Bullet {
    pub id: usize,
    position: Vec3,
    rotation: Vec3,
    live: bool,
    fire_timestamp: f32,
    rigidbody: physics::RigidBody,
}


impl Default for Bullet {
    fn default() -> Self {
        Self {
            id: 0,
            live: false,
            position: Vec3::zero(),
            rotation: Vec3::zero(),
            fire_timestamp: 0.0,
            rigidbody: physics::RigidBody::new(
                12.0, physics::moi_cube(12.0, 2.0)
            ),
        }
    }
}


impl Bullet {

    pub fn new() -> Self {
        let mut object = Self::default();
        object.id = utils::webgl_add_entity(r#"{
            "shader": "test",
            "count": 6,
            "attributes": {
                "a_position": "quad"
            },
            "uniforms": {
                "objectData": ["u_matrix"]
            }
        }"#);
        object.rigidbody.velocity_limit = 800.0;
        object
    }

    const LIFETIME: f32 = 100.0;
    const EXIT_VELOCITY: f32 = 400.0;

    pub fn fire(
        &mut self, frame: &Frame, position: Vec3, rotation: Vec3,
        dir: Vec3, parent_rb: &physics::RigidBody
    ) -> Vec3 {

        self.position = position;
        self.rotation = rotation;
        self.live = true;
        self.fire_timestamp = frame.t;

        let force = dir * self.rigidbody.mass * Self::EXIT_VELOCITY / frame.dt;
        self.rigidbody.reset();
        self.rigidbody.inherit(parent_rb);
        self.rigidbody.apply_force(force);

        -force
    }
}


impl EntityBehavior for Bullet {

    fn render_frame(&mut self, frame: &mut Frame) {
        if frame.t - self.fire_timestamp > Self::LIFETIME {
            self.live = false;
        } else {
            self.rigidbody.update_physics(
                frame.dt, &mut self.position, &mut self.rotation
            );
       }
    }

    fn update_uniforms(&mut self, frame: &mut Frame, mut matrix: Matrix4) {
        if !self.live { return; }
        matrix.translate(self.position);
        matrix.rotate(self.rotation);
        // matrix.scale(Vec3::new(5.0, 1.0, 5.0));
        frame.add_view_matrix(self.id, matrix);
    }

}
