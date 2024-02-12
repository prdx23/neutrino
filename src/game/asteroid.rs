
use crate::math::{ Vec3, Matrix4 };
use crate::physics;
use crate::engine::{ Frame };
use crate::engine::entity::{ EntityBehavior };
use crate::utils;



pub struct Asteroid {
    id: usize,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
    // pub rigidbody: physics::RigidBody,
    // pub aabb: physics::Aabb,

    pub collider: physics::collisions::PolygonCollider<4>,
    pub colliding: bool,
}


impl Default for Asteroid {
    fn default() -> Self {
        Self {
            id: 0,
            position: Vec3::zero(),
            rotation: Vec3::zero(),
            scale: Vec3::new(1.0, 1.0, 1.0),
            // rigidbody: physics::RigidBody::new(1.0, 0.0),
            // aabb: physics::Aabb::new(0.0, 0.0),
            collider: physics::collisions::PolygonCollider::new([
                Vec3::zero(),
                Vec3::zero(),
                Vec3::zero(),
                Vec3::zero(),
            ]),
            colliding: false,
        }
    }
}


impl Asteroid {

    pub fn new(s: f32) -> Self {
        let mut object = Self::default();
        object.id = utils::webgl_add_entity(r#"{
            "shader": "vertex_color",
            "count": 36,
            "attributes": {
                "a_position": "cube_vertices",
                "a_color": "cube_vertex_colors"
            },
            "uniforms": {
                "objectData": ["u_matrix", "u_collide"]
            }
        }"#);
        // object.aabb = physics::Aabb::new(s * 2.0, s * 2.0);

        // object.rigidbody = physics::RigidBody::new(
        //     250.0 * s, physics::moi_cuboid(250.0 * s, s * 2.0, s * 2.0)
        // );

        object.collider = physics::collisions::PolygonCollider::new([
            Vec3::new(-s, 0.0, -s),
            Vec3::new(s, 0.0, -s),
            Vec3::new(s, 0.0, s),
            Vec3::new(-s, 0.0, s),
        ]);
        object
    }

}


impl EntityBehavior for Asteroid {

    fn render_frame(&mut self, frame: &mut Frame) {
        // self.aabb.update(self.position);

        // self.rigidbody.apply_damping(20.0);
        // self.rigidbody.update_physics(
        //     frame.dt, &mut self.position, &mut self.rotation
        // );
    }

    fn update_uniforms(&mut self, frame: &mut Frame, mut matrix: Matrix4) {
        matrix.translate(self.position);
        matrix.rotate(self.rotation);
        self.collider.update(&matrix);

        matrix.scale(self.scale);
        frame.add_view_matrix(self.id, matrix);


        if self.colliding {
            frame.buffer.add_float(self.id, 0.0, 1.0, 1.0);
            self.colliding = false;
        } else {
            frame.buffer.add_float(self.id, 0.0, 1.0, 0.0);
        }
        // if self.aabb.colliding {
        //     frame.buffer.add_float(self.id, 0.0, 1.0, 1.0);
        // } else {
        //     frame.buffer.add_float(self.id, 0.0, 1.0, 0.0);
        // }
    }

}
