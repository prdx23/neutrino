

use crate::math::Vec3;


// Units:
//     Distance: Meter
//     mass: Kg
//     time: Second
//     velocity: Meter / Second
//     Acceleration: Meter / Second^2
//     Force: Newton



pub struct RigidBody {
    pub mass: f32,
    pub moment_of_inertia: f32,

    force: Vec3,
    velocity: Vec3,
    acceleration: Vec3,
    torque: Vec3,
    angular_velocity: Vec3,
    angular_acceleration: Vec3,

    pub velocity_limit: f32,
    pub angular_velocity_limit: f32,
}


impl RigidBody {

    pub fn new(mass: f32, moi: f32) -> Self {
        Self {
            mass: mass,
            moment_of_inertia: moi,
            force: Vec3::zero(),
            velocity: Vec3::zero(),
            acceleration: Vec3::zero(),
            torque: Vec3::zero(),
            angular_velocity: Vec3::zero(),
            angular_acceleration: Vec3::zero(),
            velocity_limit: 200.0,
            angular_velocity_limit: 7.0,
        }
    }

    pub fn reset(&mut self) {
        self.force = Vec3::zero();
        self.velocity = Vec3::zero();
        self.acceleration = Vec3::zero();
        self.torque = Vec3::zero();
        self.angular_velocity = Vec3::zero();
        self.angular_acceleration = Vec3::zero();
    }

    pub fn inherit(&mut self, other: &Self) {
        self.velocity = other.velocity;
        self.acceleration = other.acceleration;
        self.angular_velocity = other.angular_velocity;
        self.angular_acceleration = other.angular_acceleration;
    }

    pub fn apply_force(&mut self, force: Vec3) {
        self.force += force;
    }

    pub fn apply_torque(&mut self, force: Vec3, distance: Vec3) {
        self.torque += distance.cross(force);
    }

    pub fn apply_force_and_torque(&mut self, force: Vec3, distance: Vec3) {
        self.apply_force(force);
        self.apply_torque(force, distance);
    }

    pub fn apply_damping(&mut self, cof: f32) {

        if self.velocity.len() > 0.5 {
            let friction = -self.velocity.unit() * self.mass * cof;
            self.apply_force(friction);
        } else {
            self.velocity.set(0.0, 0.0, 0.0);
        }

        if self.angular_velocity.len() > 0.05 {
            self.torque += -self.angular_velocity.unit()
                * self.moment_of_inertia * cof * 0.10;
        } else {
            self.angular_velocity.set(0.0, 0.0, 0.0);
        }

    }

    pub fn update_physics(
        &mut self, dt: f32, position: &mut Vec3, rotation: &mut Vec3
    ) {

        if !(self.mass > 0.0) { return; }

        if self.velocity.is_near_zero() {
            self.velocity.set(0.0, 0.0, 0.0);
        }

        if self.angular_velocity.is_near_zero() {
            self.angular_velocity.set(0.0, 0.0, 0.0);
        }


        *position += {
            (self.velocity * dt) + (0.5 * self.acceleration * dt * dt)
        };
        self.acceleration = self.force / self.mass;
        self.velocity += 0.5 * self.acceleration * dt;


        *rotation += {
            (self.angular_velocity * dt) + (0.5 * self.angular_acceleration * dt * dt)
        };
        self.angular_acceleration = self.torque / self.moment_of_inertia;
        self.angular_velocity += 0.5 * self.angular_acceleration * dt;


        if self.velocity.len() > self.velocity_limit {
            self.velocity = self.velocity.unit() * self.velocity_limit;
        }
        if self.angular_velocity.len() > self.angular_velocity_limit {
            self.angular_velocity = {
                self.angular_velocity.unit() * self.angular_velocity_limit
            };
        }


        self.force.set(0.0, 0.0, 0.0);
        self.torque.set(0.0, 0.0, 0.0);

    }

}
