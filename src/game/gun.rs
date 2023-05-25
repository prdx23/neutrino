

use crate::math::{ Vec3, Matrix4 };
use crate::physics::{ RigidBody };
use crate::engine::{ Arena, Frame };
use crate::engine::entity::{ EntityBehavior };
use crate::game::{ Bullet };
use crate::utils;


const NBULLETS: usize = 200;


pub struct Gun {
    id: usize,
    position: Vec3,
    direction: Vec3,
    last_timestamp: f32,
    matrix: Matrix4,
    bullets: Arena<Bullet, NBULLETS>,
    bullet_index: usize,
}



impl Gun {

    pub fn new(position: Vec3, direction: Vec3) -> Self {
        let mut bullets = Arena::empty();
        for _ in 0..NBULLETS {
            bullets.add(Bullet::new());
        }
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
            position, direction,
            last_timestamp: 0.0,
            matrix: Matrix4::identity(),
            bullets: bullets,
            bullet_index: 0,
        }
    }

    const ORIGIN: Vec3 = Vec3::zero();
    const SHOOT_DELAY: f32 = 4.0;

    pub fn shoot(
        &mut self, frame: &Frame, rotation: Vec3, rigidbody: &mut RigidBody
    ) {
        if frame.t - self.last_timestamp < Self::SHOOT_DELAY { return; }

        let center = self.matrix * Self::ORIGIN;
        let distance = (self.matrix * self.position) - center;
        let direction = ((self.matrix * self.direction) - center).unit();
        let position = self.matrix * (self.position + self.direction);

        self.bullet_index = (self.bullet_index + 1) % NBULLETS;
        let bullet = &mut self.bullets[self.bullet_index.into()];
        let recoil_force = bullet.fire(
            frame, position, rotation, direction, rigidbody,
        );

        rigidbody.apply_force_and_torque(recoil_force, distance);
        self.last_timestamp = frame.t;
    }

}

impl EntityBehavior for Gun {

    fn render_frame(&mut self, frame: &mut Frame) {
        for bullet in self.bullets.slice_mut() {
            bullet.render_frame(frame);
        }
    }

    fn update_uniforms(&mut self, frame: &mut Frame, mut matrix: Matrix4) {
        self.matrix = matrix.clone();
        matrix.translate(self.position + self.direction);
        frame.add_view_matrix(self.id, matrix);

        for bullet in self.bullets.slice_mut() {
            bullet.update_uniforms(frame, Matrix4::identity());
        }
    }


}
