
use crate::math::{ Vec3, Matrix4 };
use crate::physics;
use crate::engine::entity::{ EntityBehavior };
use crate::engine::{ Key, Frame };
use crate::game::{ Thruster, Gun };
use crate::utils;



pub struct Ship {
    pub id: usize,
    pub position: Vec3,
    pub rotation: Vec3,
    rigidbody: physics::RigidBody,
    thrusters: [Thruster; 8],
    gun1: Gun,
    gun2: Gun,
}


impl Ship {


    const THRUSTER_LEFT_TOP: usize     = 0;
    const THRUSTER_RIGHT_TOP: usize    = 1;
    const THRUSTER_LEFT_BOTTOM: usize  = 2;
    const THRUSTER_RIGHT_BOTTOM: usize = 3;
    const THRUSTER_FORWARD1: usize     = 4;
    const THRUSTER_FORWARD2: usize     = 5;
    const THRUSTER_BACKWARD1: usize    = 6;
    const THRUSTER_BACKWARD2: usize    = 7;


    pub fn new() -> Self {
        Self {
            id: utils::webgl_add_entity(r#"{
                "shader": "vertex_color",
                "count": 36,
                "attributes": {
                    "a_position": "ship_vertices",
                    "a_color": "cube_vertex_colors"
                },
                "uniforms": {
                    "objectData": ["u_matrix"]
                }
            }"#),
            position: Vec3::zero(),
            rotation: Vec3::zero(),
            rigidbody: physics::RigidBody::new(
                1000.0, physics::moi_cuboid(1000.0, 4.0, 6.0)
            ),
            thrusters: [
                // THRUSTER_LEFT_TOP,
                Thruster::new(
                    Vec3::new(-4.0, 0.0, -4.0), Vec3::new(-1.0, 0.0, 0.0), 100.0,
                ),
                // THRUSTER_RIGHT_TOP,
                Thruster::new(
                    Vec3::new(4.0, 0.0, -4.0), Vec3::new(1.0, 0.0, 0.0), 100.0,
                ),
                // THRUSTER_LEFT_BOTTOM,
                Thruster::new(
                    Vec3::new(-4.0, 0.0, 4.0), Vec3::new(-1.0, 0.0, 0.0), 100.0,
                ),
                // THRUSTER_RIGHT_BOTTOM,
                Thruster::new(
                    Vec3::new(4.0, 0.0, 4.0), Vec3::new(1.0, 0.0, 0.0), 100.0,
                ),
                // THRUSTER_FORWARD1,
                Thruster::new(
                    Vec3::new(-2.0, 0.0, 6.0), Vec3::new(0.0, 0.0, 1.0), 300.0,
                ),
                // THRUSTER_FORWARD2,
                Thruster::new(
                    Vec3::new(2.0, 0.0, 6.0), Vec3::new(0.0, 0.0, 1.0), 300.0,
                ),
                // THRUSTER_BACKWARD1,
                Thruster::new(
                    Vec3::new(-1.0, 0.0, -6.0), Vec3::new(0.0, 0.0, -1.0), 100.0,
                ),
                // THRUSTER_BACKWARD2,
                Thruster::new(
                    Vec3::new(1.0, 0.0, -6.0), Vec3::new(0.0, 0.0, -1.0), 100.0,
                ),
            ],
            gun1: Gun::new(
                Vec3::new(2.0, 0.0, -5.0), Vec3::new(0.0, 0.0, -1.0)
            ),
            gun2: Gun::new(
                Vec3::new(-2.0, 0.0, -5.0), Vec3::new(0.0, 0.0, -1.0)
            ),
        }
    }

}


impl EntityBehavior for Ship {

    fn render_frame(&mut self, frame: &mut Frame) {
        let rigidbody = &mut self.rigidbody;

        if frame.pressed(Key::W) {
            self.thrusters[Self::THRUSTER_FORWARD1].fire(rigidbody, 1.0);
            self.thrusters[Self::THRUSTER_FORWARD2].fire(rigidbody, 1.0);
        }
        if frame.pressed(Key::S) {
            self.thrusters[Self::THRUSTER_BACKWARD1].fire(rigidbody, 1.0);
            self.thrusters[Self::THRUSTER_BACKWARD2].fire(rigidbody, 1.0);
        }

        if frame.pressed(Key::A) {
            self.thrusters[Self::THRUSTER_LEFT_BOTTOM].fire(rigidbody, 0.15);
            self.thrusters[Self::THRUSTER_RIGHT_TOP].fire(rigidbody, 0.15);
        }
        if frame.pressed(Key::D) {
            self.thrusters[Self::THRUSTER_LEFT_TOP].fire(rigidbody, 0.15);
            self.thrusters[Self::THRUSTER_RIGHT_BOTTOM].fire(rigidbody, 0.15);
        }

        if frame.pressed(Key::Q) {
            self.thrusters[Self::THRUSTER_LEFT_TOP].fire(rigidbody, 1.0);
            self.thrusters[Self::THRUSTER_LEFT_BOTTOM].fire(rigidbody, 1.0);
        }
        if frame.pressed(Key::E) {
            self.thrusters[Self::THRUSTER_RIGHT_TOP].fire(rigidbody, 1.0);
            self.thrusters[Self::THRUSTER_RIGHT_BOTTOM].fire(rigidbody, 1.0);
        }

        if frame.pressed(Key::Space) {
            // if frame.t % 10.0 == 0.0 {
                self.gun1.shoot(frame, self.rotation, rigidbody);
            // } else {
                self.gun2.shoot(frame, self.rotation, rigidbody);
            // }
        }

        rigidbody.apply_damping(150.0);

        rigidbody.update_physics(
            frame.dt, &mut self.position, &mut self.rotation
        );

        self.gun1.render_frame(frame);
        self.gun2.render_frame(frame);

        for thruster in self.thrusters.iter_mut() {
            thruster.render_frame(frame);
        }
    }


    fn update_uniforms(&mut self, frame: &mut Frame, mut matrix: Matrix4) {
        matrix.translate(self.position);
        matrix.rotate(self.rotation);
        frame.add_view_matrix(self.id, matrix);

        self.gun1.update_uniforms(frame, matrix);
        self.gun2.update_uniforms(frame, matrix);

        for thruster in self.thrusters.iter_mut() {
            thruster.update_uniforms(frame, matrix);
        }
    }

}
