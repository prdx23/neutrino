use crate::{math::{Vec3, Matrix4}, engine::Arena};



pub trait Collider {

    fn center(&self) -> Vec3;

    fn world_axes(&self) -> &[Vec3];

    fn project_on_axis(&self, axis: Vec3) -> (f32, f32);

    fn collide(&self, other: &dyn Collider) -> Option<(Vec3, f32)> {

        let self_center = self.center();
        let other_center = other.center();

        if f32::abs(self_center.x - other_center.x).floor() > 500.0 {
            return None;
        }

        if f32::abs(self_center.z - other_center.z).floor() > 500.0 {
            return None;
        }

        let mut self_min;
        let mut self_max;
        let mut other_min;
        let mut other_max;

        let mut sep_axis = Vec3::zero();
        let mut min_depth = f32::MAX;

        let connecting_axis = [self_center - other_center];
        let all_axes = connecting_axis
            .iter()
            .chain(self.world_axes())
            .chain(other.world_axes());

        for axis in all_axes {
            (self_min, self_max) = self.project_on_axis(*axis);
            (other_min, other_max) = other.project_on_axis(*axis);

            if self_min > other_max || self_max < other_min {
                return None;
            }

            let depth = f32::abs(self_min - other_max).min(f32::abs(self_max - other_min));
            if depth < min_depth {
                min_depth = depth;
                sep_axis = *axis;
            }

        }

        if min_depth > 0.0 {
            min_depth = min_depth / sep_axis.len();
        }

        if sep_axis.dot(connecting_axis[0]) > 0.0 {
            sep_axis = -sep_axis;
        }

        Some((sep_axis.unit(), min_depth))
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
        // TODO: need axis of closest vertex to circle center
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
    center: Vec3,
    pub vertices: [Vec3; N],
    axes: [Vec3; N],
    world_vertices: [Vec3; N],
    world_axes: [Vec3; N],
    pub matrix: Matrix4,
}


impl<const N: usize> PolygonCollider<N> {

    pub fn new(vertices: [Vec3; N]) -> Self {
        let mut obj = Self {
            vertices,
            matrix: Matrix4::identity(),
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
        self.matrix = (*matrix).clone();

        for (i, vertex) in self.vertices.iter().enumerate() {
            self.world_vertices[i] = *matrix * *vertex;
            self.world_vertices[i].y = 0.0;
        }

        for (i, axis) in self.axes.iter().enumerate() {
            self.world_axes[i] = (*matrix * *axis) - self.center;
        }
    }

    pub fn get_best_edge(&self, normal: Vec3) -> (Vec3, Vec3, Vec3) {

        let mut max = f32::MIN;
        let mut farthest_vertex = 0;

        for (i, v) in self.vertices.iter().enumerate() {
            if normal.dot(*v) > max {
                max = normal.dot(*v);
                farthest_vertex = i;
            }
        }

        let v = self.world_vertices[farthest_vertex];
        let v1 = self.world_vertices[(farthest_vertex + 1) % self.world_vertices.len()];
        let v0 = self.world_vertices[(farthest_vertex + self.world_vertices.len() - 1) % self.world_vertices.len()];

        let left = (v - v1).unit();
        let right = (v - v0).unit();

        if right.dot(normal) <= left.dot(normal) {
            (v, v0, v)
        } else {
            (v, v, v1)
        }
    }

}


pub fn clip(v1: Vec3, v2: Vec3, normal: Vec3, offset: f32) -> Arena<Vec3, 2> {
    // let mut p1 = Vec3::zero();
    // let mut p2 = None;

    let mut points = Arena::empty();

    let d1 = normal.dot(v1) - offset;
    let d2 = normal.dot(v2) - offset;

    if d1 > 0.0 {
        points.add(v1);
    }

    if d2 > 0.0 {
        points.add(v2);
    }

    if d1 * d2 < 0.0 {
        let mut e = v2 - v1;
        let u = d1 / (d1 - d2);
        e *= u;
        e += v1;
        points.add(e);
    }

    points

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


// pub enum CollisionType {
//     Ship,
//     ShipBullet,
//     Asteroid,
// }

// pub trait CollisionBehavior {

//     fn object_type(&self) -> CollisionType;

//     fn collider(&self) -> &dyn Collider;

//     fn collide(&mut self, other: &mut dyn CollisionBehavior);

//     fn handle_collision(&mut self, ctype: CollisionType);

// }
