use crate::math::{Vec3, Matrix4};



pub trait Collider {

    fn center(&self) -> Vec3;

    fn world_axes(&self) -> &[Vec3];

    fn project_on_axis(&self, axis: Vec3) -> (f32, f32);

    fn collide(&self, other: &dyn Collider) -> bool {

        let self_center = self.center();
        let other_center = other.center();

        if f32::abs(self_center.x - other_center.x).floor() > 500.0 {
            return false;
        }

        if f32::abs(self_center.z - other_center.z).floor() > 500.0 {
            return false;
        }

        let mut self_min;
        let mut self_max;
        let mut other_min;
        let mut other_max;

        let connecting_axis = [self_center - other_center];
        let all_axes = connecting_axis
            .iter()
            .chain(self.world_axes())
            .chain(other.world_axes());

        for axis in all_axes {
            (self_min, self_max) = self.project_on_axis(*axis);
            (other_min, other_max) = other.project_on_axis(*axis);

            if self_min > other_max || self_max < other_min {
                return false;
            }
        }

        true
    }

}


pub struct CircleCollider {
    center: Vec3,
    radius: f32,
}


impl CircleCollider {

    pub fn new(radius: f32) -> Self {
        Self { center: Vec3::zero(), radius }
    }

    pub fn update(&mut self, position: Vec3) {
        self.center = position;
    }

}


impl Collider for CircleCollider {

    fn center(&self) -> Vec3 {
        self.center
    }

    fn world_axes(&self) -> &[Vec3] {
        &[]
    }

    fn project_on_axis(&self, axis: Vec3) -> (f32, f32) {
        let dot = self.center.dot(axis);
        let a = dot - self.radius;
        let b = dot + self.radius;
        (a.min(b), a.max(b))
    }

}


pub struct PolygonCollider<const N: usize> {
    pub center: Vec3,
    pub vertices: [Vec3; N],
    pub axes: [Vec3; N],
    pub world_vertices: [Vec3; N],
    pub world_axes: [Vec3; N],
}


impl<const N: usize> PolygonCollider<N> {

    pub fn new(vertices: [Vec3; N]) -> Self {
        let mut obj = Self {
            vertices,
            center: Vec3::zero(),
            axes: core::array::from_fn(|_| Vec3::zero()),
            world_vertices: core::array::from_fn(|_| Vec3::zero()),
            world_axes: core::array::from_fn(|_| Vec3::zero()),
        };

        for i in 0..N {
            let edge = obj.vertices[i] - obj.vertices[((i + 1) % N)];
            obj.axes[i] = Vec3::new(-edge.z, 0.0, edge.x);
        }

        obj
    }


    const ORIGIN: Vec3 = Vec3::zero();

    pub fn update(&mut self, matrix: &Matrix4) {
        self.center = *matrix * Self::ORIGIN;

        for (i, vertex) in self.vertices.iter().enumerate() {
            self.world_vertices[i] = *matrix * *vertex;
            self.world_vertices[i].y = 0.0;
        }

        for (i, axis) in self.axes.iter().enumerate() {
            self.world_axes[i] = (*matrix * *axis) - self.center;
        }
    }

}


impl<const N: usize> Collider for PolygonCollider<N> {

    fn center(&self) -> Vec3 {
        self.center
    }

    fn world_axes(&self) -> &[Vec3] {
        &self.world_axes
    }

    fn project_on_axis(&self, axis: Vec3) -> (f32, f32) {
        let mut min = f32::MAX;
        let mut max = f32::MIN;
        let mut dot;

        for vertex in self.world_vertices.iter() {
            dot = vertex.dot(axis);
            min = dot.min(min);
            max = dot.max(max);
        }

        (min, max)
    }

}
