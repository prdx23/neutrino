

use crate::math::Vec3;


pub struct RigidBody {
    enabled: bool,

    pub mass: f32,
    pub damping_factor: f32,
    // pub coff_of_friction: f32,
    pub moment_of_inertia: f32,

    force: Vec3,
    pub velocity: Vec3,
    acceleration: Vec3,
    torque: Vec3,
    angular_velocity: Vec3,
    angular_acceleration: Vec3,
}


impl Default for RigidBody {
    fn default() -> Self {
        Self {
            enabled: false,
            mass: 0.0,
            damping_factor: 0.0,
            // coff_of_friction: cof,
            moment_of_inertia: 0.0,
            force: Vec3::zero(),
            velocity: Vec3::zero(),
            acceleration: Vec3::zero(),
            torque: Vec3::zero(),
            angular_velocity: Vec3::zero(),
            angular_acceleration: Vec3::zero(),
        }
    }
}


impl RigidBody {

    pub fn enable(&mut self, mass: f32, damping: f32) {
        self.enabled = true;
        self.mass = mass;
        self.damping_factor = damping;
        self.moment_of_inertia = 10.0 * 10.0 * mass / 6.0;
    }

    pub fn apply_force(&mut self, force: Vec3) {
        self.force += force;
    }

    pub fn apply_force_comps(&mut self, x: f32, y: f32, z: f32) {
        self.force.x += x;
        self.force.y += y;
        self.force.z += z;
    }

    pub fn apply_force_at_pos(&mut self, force: Vec3, pos: Vec3) {
        self.force += force;
        self.torque += pos.cross(force);
    }

    pub fn update_physics(
        &mut self, mut position: Vec3, mut rotation: Vec3, dt: f32
    ) -> (Vec3, Vec3) {

        if !self.enabled { return (position, rotation); }
        if !(self.mass > 0.0) { return (position, rotation); }


        if self.velocity.is_near_zero() {
            self.velocity.set(0.0, 0.0, 0.0);
        } else {
            self.apply_force(self.velocity * self.damping_factor * -1.0);
        }

        if self.angular_velocity.is_near_zero() {
            self.angular_velocity.set(0.0, 0.0, 0.0);
        } else {
            self.torque += self.angular_velocity * self.moment_of_inertia * -0.01;
        }


        let velocity_half = self.velocity + (0.5 * self.acceleration * dt * dt);
        position = position + (velocity_half * dt);
        self.acceleration = self.force / self.mass;
        self.velocity = velocity_half + (0.5 * self.acceleration * dt * dt);
        self.force.set(0.0, 0.0, 0.0);


        let ang_velocity_half = {
            self.angular_velocity + (0.5 * self.angular_acceleration * dt * dt)
        };
        rotation = rotation + (ang_velocity_half * dt);
        self.angular_acceleration = self.torque / self.moment_of_inertia;
        self.angular_velocity = {
            ang_velocity_half + (0.5 * self.angular_acceleration * dt * dt)
        };
        self.torque.set(0.0, 0.0, 0.0);

        (position, rotation)
    }

}
