
use crate::math::{ Vec3, Matrix4 };
use crate::physics::{ RigidBody };
use crate::engine::entity::{ EntityBehavior };
use crate::engine::{ Key, Frame };
use crate::game::{ Thruster };
use crate::utils;



pub struct Ship {
    pub id: usize,
    pub position: Vec3,
    pub rotation: Vec3,
    rigidbody: RigidBody,
    thrusters: [Thruster; 8],
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
        let ship = Self {
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
            rigidbody: RigidBody::new(100.0, 2.0),
            thrusters: [
                // THRUSTER_LEFT_TOP,
                Thruster::new(
                    Vec3::new(-4.0, 0.0, -4.0), Vec3::new(1.0, 0.0, 0.0), 1.1,
                ),
                // THRUSTER_RIGHT_TOP,
                Thruster::new(
                    Vec3::new(4.0, 0.0, -4.0), Vec3::new(-1.0, 0.0, 0.0), 1.1,
                ),
                // THRUSTER_LEFT_BOTTOM,
                Thruster::new(
                    Vec3::new(-4.0, 0.0, 4.0), Vec3::new(1.0, 0.0, 0.0), 1.1,
                ),
                // THRUSTER_RIGHT_BOTTOM,
                Thruster::new(
                    Vec3::new(4.0, 0.0, 4.0), Vec3::new(-1.0, 0.0, 0.0), 1.1,
                ),
                // THRUSTER_FORWARD1,
                Thruster::new(
                    Vec3::new(-2.0, 0.0, 6.0), Vec3::new(0.0, 0.0, -1.0), 1.7,
                ),
                // THRUSTER_FORWARD2,
                Thruster::new(
                    Vec3::new(2.0, 0.0, 6.0), Vec3::new(0.0, 0.0, -1.0), 1.7,
                ),
                // THRUSTER_BACKWARD1,
                Thruster::new(
                    Vec3::new(-1.0, 0.0, -6.0), Vec3::new(0.0, 0.0, 1.0), 1.3,
                ),
                // THRUSTER_BACKWARD2,
                Thruster::new(
                    Vec3::new(1.0, 0.0, -6.0), Vec3::new(0.0, 0.0, 1.0), 1.3,
                ),
            ],
            // gun: Default::default(),
        };

        // let gun = scenegraph.add_entity(shipid, Gun::new(
        //     Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, -1.0)
        // ).into());

        ship
    }

}


impl EntityBehavior for Ship {

    fn render_frame(&mut self, frame: &mut Frame) {

        if frame.pressed(Key::W) {
            self.thrusters[Self::THRUSTER_FORWARD1].fire(&mut self.rigidbody);
            self.thrusters[Self::THRUSTER_FORWARD2].fire(&mut self.rigidbody);
        }
        if frame.pressed(Key::S) {
            self.thrusters[Self::THRUSTER_BACKWARD1].fire(&mut self.rigidbody);
            self.thrusters[Self::THRUSTER_BACKWARD2].fire(&mut self.rigidbody);
        }

        if frame.pressed(Key::A) {
            self.thrusters[Self::THRUSTER_LEFT_BOTTOM].fire(&mut self.rigidbody);
            self.thrusters[Self::THRUSTER_RIGHT_TOP].fire(&mut self.rigidbody);
        }
        if frame.pressed(Key::D) {
            self.thrusters[Self::THRUSTER_LEFT_TOP].fire(&mut self.rigidbody);
            self.thrusters[Self::THRUSTER_RIGHT_BOTTOM].fire(&mut self.rigidbody);
        }

        if frame.pressed(Key::Q) {
            self.thrusters[Self::THRUSTER_LEFT_TOP].fire(&mut self.rigidbody);
            self.thrusters[Self::THRUSTER_LEFT_BOTTOM].fire(&mut self.rigidbody);
        }
        if frame.pressed(Key::E) {
            self.thrusters[Self::THRUSTER_RIGHT_TOP].fire(&mut self.rigidbody);
            self.thrusters[Self::THRUSTER_RIGHT_BOTTOM].fire(&mut self.rigidbody);
        }

        // if frame.pressed(Key::Space) {
        //     let mut gun: Gun = scenegraph.load(self.gun).into();
        //     gun.shoot(t, &mut self.rigidbody, scenegraph, self.gun);
        //     scenegraph.store(self.gun, gun.into());
        // }

        self.rigidbody.update_physics(
            frame.dt, &mut self.position, &mut self.rotation
        );

        // for thruster in self.thrusters.iter_mut() {
        //     thruster.render_frame(frame);
        // }
    }


    fn update_uniforms(&mut self, frame: &mut Frame, mut matrix: Matrix4) {
        matrix.translate(self.position);
        matrix.rotate(self.rotation);
        frame.add_view_matrix(self.id, matrix);

        for thruster in self.thrusters.iter_mut() {
            thruster.update_uniforms(frame, matrix);
        }
    }

}
