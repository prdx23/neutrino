use std::ops::{ Mul, MulAssign };
use crate::math::Vec3;


#[derive(Clone, Copy, PartialEq)]
pub struct Matrix4 {
    pub matrix: [[f32; 4]; 4],
}


impl Matrix4 {

    pub fn identity() -> Self {
        Self {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    fn translation(tx: f32, ty: f32, tz: f32) -> Self {
        Self {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ tx,  ty,  tz, 1.0],
            ],
        }
    }

    pub fn translate(&mut self, pos: Vec3) {
        *self *= Matrix4::translation(pos.x, pos.y, pos.z);
    }

    pub fn translate_x(&mut self, tx: f32) {
        *self *= Matrix4::translation(tx, 0.0, 0.0);
    }

    pub fn translate_y(&mut self, ty: f32) {
        *self *= Matrix4::translation(0.0, ty, 0.0);
    }

    pub fn translate_z(&mut self, tz: f32) {
        *self *= Matrix4::translation(0.0, 0.0, tz);
    }


    fn scaling(sx: f32, sy: f32, sz: f32) -> Self {
        Self {
            matrix: [
                [ sx, 0.0, 0.0, 0.0],
                [0.0,  sy, 0.0, 0.0],
                [0.0, 0.0,  sz, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }


    pub fn scale(&mut self, scales: Vec3) {
        *self *= Matrix4::scaling(scales.x, scales.y, scales.z);
    }

    pub fn scale_x(&mut self, sx: f32) {
        *self *= Matrix4::scaling(sx, 1.0, 1.0);
    }

    pub fn scale_y(&mut self, sy: f32) {
        *self *= Matrix4::scaling(1.0, sy, 1.0);
    }

    pub fn scale_z(&mut self, sz: f32) {
        *self *= Matrix4::scaling(1.0, 1.0, sz);
    }


    fn rotation_x(theta: f32) -> Self {
        // assumption: theta is radians
        let sin = theta.sin();
        let cos = theta.cos();
        Self {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, cos, sin, 0.0],
                [0.0,-sin, cos, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotate_x(&mut self, theta: f32) {
        *self *= Matrix4::rotation_x(theta);
    }

    fn rotation_y(theta: f32) -> Self {
        // assumption: theta is radians
        let sin = theta.sin();
        let cos = theta.cos();
        Self {
            matrix: [
                [cos, 0.0,-sin, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [sin, 0.0, cos, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotate_y(&mut self, theta: f32) {
        *self *= Matrix4::rotation_y(theta);
    }

    fn rotation_z(theta: f32) -> Self {
        // assumption: theta is radians
        let sin = theta.sin();
        let cos = theta.cos();
        Self {
            matrix: [
                [ cos, sin, 0.0, 0.0],
                [-sin, cos, 0.0, 0.0],
                [ 0.0, 0.0, 1.0, 0.0],
                [ 0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotate_z(&mut self, theta: f32) {
        *self *= Matrix4::rotation_z(theta);
    }


    pub fn rotate(&mut self, angles: Vec3) {
        self.rotate_x(angles.x);
        self.rotate_y(angles.y);
        self.rotate_z(angles.z);
    }


    pub fn projection(width: f32, height: f32, depth: f32) -> Self {
        Self {
            matrix: [
                [2.0 / width, 0.0, 0.0, 0.0],
                [0.0, -2.0 / height, 0.0, 0.0],
                [0.0, 0.0, 2.0 / depth, 0.0],
                [-1.0, 1.0, 0.0, 1.0],
            ],
        }
    }

    pub fn orthographic(
        left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32
    ) -> Self {
        Self {
            matrix: [
                [2.0 / (right - left), 0.0, 0.0, 0.0],
                [0.0, 2.0 / (top - bottom), 0.0, 0.0],
                [0.0, 0.0, 2.0 / (near - far), 0.0],
                [
                    (left + right) / (left - right),
                    (bottom + top) / (bottom - top),
                    (near + far) / (near - far),
                    1.0,
                ],
            ],
        }
    }

    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        // assumption: fov is degrees
        let f = (0.5 * crate::PI - 0.5 * fov.to_radians()).tan();
        let range_inv = 1.0 / (near - far);
        Self {
            matrix: [
                [f / aspect, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (near + far) * range_inv, -1.0],
                [0.0, 0.0, near * far * range_inv * 2.0, 1.0],
            ],
        }
    }

    pub fn look_at(camera_pos: Vec3, target: Vec3, up: Vec3) -> Self {
        let z_axis = (camera_pos - target).unit();
        let x_axis = (up.cross(z_axis)).unit();
        let y_axis = (z_axis.cross(x_axis)).unit();
        Self {
            matrix: [
                [x_axis.x, x_axis.y, x_axis.z, 0.0],
                [y_axis.x, y_axis.y, y_axis.z, 0.0],
                [z_axis.x, z_axis.y, z_axis.z, 0.0],
                [camera_pos.x, camera_pos.y, camera_pos.z, 1.0],
            ],
        }
    }



    pub fn cofactor(
        mat: [[f32; 4]; 4], p: usize, q: usize, n: usize
    ) -> [[f32; 4]; 4] {

        let mut temp = [[0.0; 4]; 4];

        let mut i = 0;
        let mut j = 0;

        for row in 0..n {
            for col in 0..n {
                if row != p && col != q {
                    temp[i][j] = mat[row][col];
                    j += 1;
                    if j == n - 1 {
                        j = 0;
                        i += 1;
                    }
                }
            }
        }

        temp
    }


    pub fn determinant(mat: [[f32; 4]; 4], n: usize) -> f32 {
        if n == 1 { return mat[0][0] }

        let mut det = 0.0;
        let mut sign = 1.0;
        for f in 0..n {
            let cofactor = Matrix4::cofactor(mat, 0, f, n);
            det += sign * mat[0][f] * Matrix4::determinant(cofactor, n - 1);
            sign = sign * -1.0;
        }
        det
    }


    pub fn adjoint(mat: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
        let mut adj = [[0.0; 4]; 4];
        let mut sign;
        for i in 0..4 {
            for j in 0..4 {
                let cofactor = Matrix4::cofactor(mat, i, j, 4);
                sign = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
                adj[j][i] = sign * Matrix4::determinant(cofactor, 3);
            }
        }
        adj
    }


    pub fn inverse(&self) -> Self {
        let mut inverse = [[0.0; 4]; 4];
        let det = Matrix4::determinant(self.matrix, 4);
        let adj = Matrix4::adjoint(self.matrix);

        debug_assert!(det > 0.0);
        for i in 0..4 {
            for j in 0..4 {
                inverse[i][j] = adj[i][j] / det;
            }
        }
        Self { matrix: inverse }
    }


    pub fn transpose(&self) -> Self {
        let mut transpose = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                transpose[i][j] = self.matrix[j][i];
            }
        }

        Self { matrix: transpose }
    }
}


impl MulAssign for Matrix4 {
    fn mul_assign(&mut self, other: Matrix4) {
        let mut res = Matrix4 { matrix: [[0.0; 4]; 4] };
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    res.matrix[i][j] += other.matrix[i][k] * self.matrix[k][j];
                }
            }
        }
        self.matrix = res.matrix
    }
}


impl Mul for Matrix4 {
    type Output = Self;
    fn mul(self, other: Matrix4) -> Matrix4 {
        let mut res = Matrix4 { matrix: [[0.0; 4]; 4] };
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    res.matrix[i][j] += other.matrix[i][k] * self.matrix[k][j];
                }
            }
        }
        res
    }
}



impl Mul<Vec3> for Matrix4 {
    type Output = Vec3;
    fn mul(self, p: Vec3) -> Vec3 {
        let mut result = Vec3::zero();
        for i in 0..3 {
            result[i] = (self.matrix[i][0] * p.x)
                      + (self.matrix[i][1] * p.y)
                      + (self.matrix[i][2] * p.z)
                      +  self.matrix[i][3];
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_multiply() {
        let matrix1 = Matrix4 {
            matrix: [
                [2.0, 4.0, 1.0, 3.0],
                [3.0, 5.0, 7.0, 8.0],
                [7.0, 8.0, 9.0, 9.0],
                [3.0, 8.0, 7.0, 3.0],
            ]
        };
        let matrix2 = Matrix4 {
            matrix: [
                [4.0, 2.0, 0.0, 9.0],
                [5.0, 6.0, 1.0, 7.0],
                [7.0, 7.0, 4.0, 3.0],
                [9.0, 9.0, 3.0, 8.0],
            ]
        };
        let result = matrix1 * matrix2;
        assert_eq!(result.matrix, [
            [62.0, 62.0, 17.0, 73.0],
            [158.0, 157.0, 57.0, 147.0],
            [212.0, 206.0, 71.0, 218.0],
            [128.0, 130.0, 45.0, 128.0],
        ]);
    }

    #[test]
    fn matrix_multiply_assign() {
        let mut matrix1 = Matrix4 {
            matrix: [
                [2.0, 4.0, 1.0, 3.0],
                [3.0, 5.0, 7.0, 8.0],
                [7.0, 8.0, 9.0, 9.0],
                [3.0, 8.0, 7.0, 3.0],
            ]
        };
        let matrix2 = Matrix4 {
            matrix: [
                [4.0, 2.0, 0.0, 9.0],
                [5.0, 6.0, 1.0, 7.0],
                [7.0, 7.0, 4.0, 3.0],
                [9.0, 9.0, 3.0, 8.0],
            ]
        };
        matrix1 *= matrix2;
        assert_eq!(matrix1.matrix, [
            [62.0, 62.0, 17.0, 73.0],
            [158.0, 157.0, 57.0, 147.0],
            [212.0, 206.0, 71.0, 218.0],
            [128.0, 130.0, 45.0, 128.0],
        ]);
    }


    #[test]
    fn matrix_vector_multiply() {
        let matrix = Matrix4 {
            matrix: [
                [2.0, 4.0, 1.0, 3.0],
                [3.0, 5.0, 7.0, 8.0],
                [7.0, 8.0, 9.0, 9.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        };
        let vector = Vec3::new(23.0, 42.0, 125.0);
        let result = matrix * vector;
        assert_eq!(Vec3::new(342.0, 1162.0, 1631.0), result);
    }

    #[test]
    fn matrix_determinant() {
        let matrix = [
            [1.0, 0.0, 2.0, -1.0],
            [3.0, 0.0, 0.0, 5.0],
            [2.0, 1.0, 4.0, -3.0],
            [1.0, 0.0, 5.0, 0.0],
        ];
        assert_eq!(Matrix4::determinant(matrix, 4), 30.0);
    }

    #[test]
    fn matrix_inverse() {

        let matrix = Matrix4 {
            matrix: [
                [2.0, 4.0, 1.0, 3.0],
                [3.0, 5.0, 7.0, 8.0],
                [7.0, 8.0, 9.0, 9.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        };
        assert_eq!(matrix.inverse(), Matrix4 {
            matrix: [
                [-1.0 / 5.0, -28.0 / 55.0, 23.0 / 55.0, 10.0 / 11.0],
                [2.0 / 5.0, 1.0 / 5.0, -1.0 / 5.0, -1.0],
                [-1.0 / 5.0, 12.0 / 55.0, -2.0 / 55.0, -9.0 / 11.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        });
    }

    #[test]
    fn matrix_transpose() {

        let matrix = Matrix4 {
            matrix: [
                [2.0, 4.0, 1.0, 3.0],
                [3.0, 5.0, 7.0, 8.0],
                [7.0, 8.0, 9.0, 9.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        };
        assert_eq!(matrix.transpose().matrix, [
            [2.0, 3.0, 7.0, 0.0],
            [4.0, 5.0, 8.0, 0.0],
            [1.0, 7.0, 9.0, 0.0],
            [3.0, 8.0, 9.0, 1.0],
        ]);
    }
}

