use std::ops::{
    Add, Sub, Neg, Mul, Div,
    AddAssign, SubAssign, MulAssign, DivAssign,
    Index, IndexMut,
};


#[derive(Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}


impl Vec3 {

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn zero() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    // pub fn inf() -> Vec3 {
    //     Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY)
    // }

    // pub fn neg_inf() -> Vec3 {
    //     Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY)
    // }

    // --------------------------------------------------------

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self {
        Vec3 {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
        }
    }

    pub fn len(&self) -> f32 {
        self.sq_len().sqrt()
    }

    pub fn sq_len(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit(&self) -> Self {
        *self / self.len()
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(mut self, other: Self) -> Self {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(mut self) -> Self {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(mut self, other: f32) -> Self {
        self.x *= other;
        self.y *= other;
        self.z *= other;
        self
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, mut other: Vec3) -> Vec3 {
        other.x *= self;
        other.y *= self;
        other.z *= self;
        other
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(mut self, other: Vec3) -> Self {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
        self
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}


impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(mut self, other: f32) -> Self {
        self.x /= other;
        self.y /= other;
        self.z /= other;
        self
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        debug_assert!(i < 3);
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => &self.z,
            // _ => panic!("Out of bounds for vector"),
        }
    }
}


impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        debug_assert!(i < 3);
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => &mut self.z,
            // _ => panic!("Out of bounds for vector"),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_vector() {
        let vector = Vec3::new(-32.0, 48.0, 100079.123);
        assert_eq!(vector.x, -32.0);
        assert_eq!(vector.y, 48.0);
        assert_eq!(vector.z, 100079.123);
    }

    #[test]
    fn zero_vector() {
        let vector = Vec3::zero();
        assert_eq!(vector.x, 0.0);
        assert_eq!(vector.y, 0.0);
        assert_eq!(vector.z, 0.0);
    }

    #[test]
    fn vector_set() {
        let vector = Vec3::zero();
        vector.set(1.0, 2.0, 3.0);
        assert_eq!(vector.x, 1.0);
        assert_eq!(vector.y, 2.0);
        assert_eq!(vector.z, 3.0);
    }

    #[test]
    fn vector_add() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let vector2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(Vec3 { x:5.0, y:7.0, z:9.0 }, vector1 + vector2);
    }

    #[test]
    fn vector_add_assign() {
        let mut vector1 = Vec3::new(1.0, 2.0, 3.0);
        vector1 += Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(Vec3 { x:5.0, y:7.0, z:9.0 }, vector1);
    }

    #[test]
    fn vector_sub() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let vector2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(Vec3 { x:3.0, y:3.0, z:3.0 }, vector2 - vector1);
    }

    #[test]
    fn vector_sub_assign() {
        let mut vector1 = Vec3::new(4.0, 5.0, 6.0);
        vector1 -= Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Vec3 { x:3.0, y:3.0, z:3.0 }, vector1);
    }

    #[test]
    fn vector_neg() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Vec3 { x:-1.0, y:-2.0, z:-3.0 }, -vector1);
    }

    #[test]
    fn vector_mul() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Vec3 { x:2.0, y:4.0, z:6.0 }, vector1 * 2.0);
    }

    #[test]
    fn vector_mul_assign() {
        let mut vector1 = Vec3::new(1.0, 2.0, 3.0);
        vector1 *= 2.0;
        assert_eq!(Vec3 { x:2.0, y:4.0, z:6.0 }, vector1);
    }

    #[test]
    fn vector_mul_rev() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Vec3 { x:2.0, y:4.0, z:6.0 }, 2.0 * vector1);
    }

    #[test]
    fn vector_mul_vector() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let vector2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(Vec3 { x:4.0, y:10.0, z:18.0 }, vector1 * vector2);
    }

    #[test]
    fn vector_mul_assign_vector() {
        let mut vector1 = Vec3::new(1.0, 2.0, 3.0);
        vector1 *= Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(Vec3 { x:4.0, y:10.0, z:18.0 }, vector1);
    }

    #[test]
    fn vector_div() {
        let vector1 = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(Vec3 { x:1.0, y:2.0, z:3.0 }, vector1 / 2.0);
    }

    #[test]
    fn vector_div_assign() {
        let mut vector1 = Vec3::new(2.0, 4.0, 6.0);
        vector1 /= 2.0;
        assert_eq!(Vec3 { x:1.0, y:2.0, z:3.0 }, vector1);
    }

    #[test]
    fn vector_dot() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let vector2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(32.0, vector1.dot(vector2));
    }

    #[test]
    fn vector_cross() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let vector2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(Vec3 { x:-3.0, y:6.0, z:-3.0 }, vector1.cross(vector2));
    }

    #[test]
    fn vector_len() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(14.0, vector1.sq_len());
        assert_eq!((14.0 as f32).sqrt(), vector1.len());
    }

    // needs float compare with <= e
    // #[test]
    // fn vector_unit() {
    //     let vector1 = Vec3::new(1.0, 2.0, 3.0);
    //     assert_eq!(Vec3 { x:2.0, y:4.0, z:6.0 }, vector1.unit());
    // }

    #[test]
    fn vector_index() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(1.0, vector1[0]);
        assert_eq!(2.0, vector1[1]);
        assert_eq!(3.0, vector1[2]);
    }

    // #[test]
    // #[should_panic(expected = "Out of bounds for vector")]
    // fn vector_index_oob() {
    //     let vector1 = Vec3::new(1.0, 2.0, 3.0);
    //     assert_eq!(1.0, vector1[4]);
    // }

    #[test]
    fn vector_index_mut() {
        let mut vector1 = Vec3::new(1.0, 2.0, 3.0);
        vector1[0] += 1.0;
        vector1[1] += 1.0;
        vector1[2] += 1.0;
        assert_eq!(2.0, vector1[0]);
        assert_eq!(3.0, vector1[1]);
        assert_eq!(4.0, vector1[2]);
    }

    // #[test]
    // #[should_panic(expected = "Out of bounds for vector")]
    // fn vector_index_mut_oob() {
    //     let mut vector1 = Vec3::new(1.0, 2.0, 3.0);
    //     vector1[4] += 1.0;
    //     assert_eq!(1.0, vector1[1]);
    // }
}
