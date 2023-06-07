
pub mod rigidbody;
pub use rigidbody::RigidBody;

pub mod aabb;
pub use aabb::Aabb;

pub mod collisions;

pub fn moi_cube(m: f32, s: f32) -> f32 {
    (1.0 / 6.0) * m * s * s
}

pub fn moi_cuboid(m: f32, a: f32, b: f32) -> f32 {
    (1.0 / 12.0) * m * ((a * a) + (b * b))
}
