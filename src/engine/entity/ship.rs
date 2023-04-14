

use crate::math::{ Vec3, Matrix4 };
use crate::physics::{ RigidBody };
use crate::engine::{ Arena };
use crate::engine::entity::{ EntityBehavior, Thruster };
use crate::engine::scenegraph::{ NodeID, Scenegraph };
use crate::utils::{ Keys };



pub struct Ship {
    pub position: Vec3,
    pub rotation: Vec3,
    rigidbody: RigidBody,
    engines: Arena<NodeID, 8>,
}


impl Ship {

    const META: &'static str = r#"{
        "shader": "vertex_color",
        "count": 36,
        "attributes": {
            "a_position": "ship_vertices",
            "a_color": "cube_vertex_colors"
        },
        "uniforms": {
            "objectData": ["u_matrix"]
        }
    }"#;

    const ENGINE_LEFT_TOP: usize     = 0;
    const ENGINE_RIGHT_TOP: usize    = 1;
    const ENGINE_LEFT_BOTTOM: usize  = 2;
    const ENGINE_RIGHT_BOTTOM: usize = 3;
    const ENGINE_FORWARD1: usize     = 4;
    const ENGINE_FORWARD2: usize     = 5;
    const ENGINE_BACKWARD1: usize    = 6;
    const ENGINE_BACKWARD2: usize    = 7;


    pub fn new(root: NodeID, scenegraph: &mut Scenegraph) -> NodeID {
        let shipid = scenegraph.add_entity(root, Self {
            position: Vec3::zero(),
            rotation: Vec3::zero(),
            rigidbody: RigidBody::new(100.0, 2.0),
            engines: Arena::empty(),
        }.into());

        let engines = [
            // ENGINE_LEFT_TOP,
            scenegraph.add_entity(shipid, Thruster::new(
                Vec3::new(-4.0, 0.0, -4.0), Vec3::new(1.0, 0.0, 0.0), 1.1,
            ).into()),

            // ENGINE_RIGHT_TOP,
            scenegraph.add_entity(shipid, Thruster::new(
                Vec3::new(4.0, 0.0, -4.0), Vec3::new(-1.0, 0.0, 0.0), 1.1,
            ).into()),

            // ENGINE_LEFT_BOTTOM,
            scenegraph.add_entity(shipid, Thruster::new(
                Vec3::new(-4.0, 0.0, 4.0), Vec3::new(1.0, 0.0, 0.0), 1.1,
            ).into()),

            // ENGINE_RIGHT_BOTTOM,
            scenegraph.add_entity(shipid, Thruster::new(
                Vec3::new(4.0, 0.0, 4.0), Vec3::new(-1.0, 0.0, 0.0), 1.1,
            ).into()),

            // ENGINE_FORWARD1,
            scenegraph.add_entity(shipid, Thruster::new(
                Vec3::new(-2.0, 0.0, 6.0), Vec3::new(0.0, 0.0, -1.0), 1.7,
            ).into()),

            // ENGINE_FORWARD2,
            scenegraph.add_entity(shipid, Thruster::new(
                Vec3::new(2.0, 0.0, 6.0), Vec3::new(0.0, 0.0, -1.0), 1.7,
            ).into()),

            // ENGINE_BACKWARD1,
            scenegraph.add_entity(shipid, Thruster::new(
                Vec3::new(-1.0, 0.0, -6.0), Vec3::new(0.0, 0.0, 1.0), 1.3,
            ).into()),

            // ENGINE_BACKWARD2,
            scenegraph.add_entity(shipid, Thruster::new(
                Vec3::new(1.0, 0.0, -6.0), Vec3::new(0.0, 0.0, 1.0), 1.3,
            ).into()),
        ];

        scenegraph.with(shipid, |mut ship: Self| {
            ship.engines = Arena::from(engines);
            ship
        });

        shipid
    }

    pub fn fire_engine(&mut self, scenegraph: &Scenegraph, location: usize) {
        scenegraph.with(self.engines[location], |mut engine: Thruster| {
            engine.fire(&mut self.rigidbody);
            engine
        });
    }

    pub fn update_frame(&mut self, keybyte: u8, scenegraph: &Scenegraph) {

        if Keys::pressed(keybyte, Keys::W) {
            self.fire_engine(scenegraph, Self::ENGINE_FORWARD1);
            self.fire_engine(scenegraph, Self::ENGINE_FORWARD2);
        }
        if Keys::pressed(keybyte, Keys::S) {
            self.fire_engine(scenegraph, Self::ENGINE_BACKWARD1);
            self.fire_engine(scenegraph, Self::ENGINE_BACKWARD2);
        }


        if Keys::pressed(keybyte, Keys::A) {
            self.fire_engine(scenegraph, Self::ENGINE_LEFT_BOTTOM);
            self.fire_engine(scenegraph, Self::ENGINE_RIGHT_TOP);
        }
        if Keys::pressed(keybyte, Keys::D) {
            self.fire_engine(scenegraph, Self::ENGINE_LEFT_TOP);
            self.fire_engine(scenegraph, Self::ENGINE_RIGHT_BOTTOM);
        }

        if Keys::pressed(keybyte, Keys::Q) {
            self.fire_engine(scenegraph, Self::ENGINE_LEFT_TOP);
            self.fire_engine(scenegraph, Self::ENGINE_LEFT_BOTTOM);
        }
        if Keys::pressed(keybyte, Keys::E) {
            self.fire_engine(scenegraph, Self::ENGINE_RIGHT_TOP);
            self.fire_engine(scenegraph, Self::ENGINE_RIGHT_BOTTOM);
        }

    }

}


impl EntityBehavior for Ship {

    fn update_matrix(&mut self, dt: f32, mut matrix: Matrix4) -> Matrix4 {
        self.rigidbody.update_physics(
            dt, &mut self.position, &mut self.rotation
        );
        matrix.translate(self.position);
        matrix.rotate(self.rotation);
        matrix
    }

    fn shader_metadata(&self) -> Option<&'static str> {
        Some(Self::META)
    }

}
