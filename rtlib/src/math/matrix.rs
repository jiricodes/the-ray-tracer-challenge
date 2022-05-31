use crate::math::vec4::Vec4;
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
    pub data: [[f32; 4]; 4],
}

impl Mat4 {
    pub const ZERO: Self = Self {
        data: [[0.0; 4]; 4],
    };
    pub const IDENTITY: Self = Self {
        data: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };
    pub fn get(&self, r: usize, c: usize) -> f32 {
        self.data[r][c]
    }

    pub fn set(&mut self, r: usize, c: usize, val: f32) {
        self.data[r][c] = val;
    }

    pub fn transpose(&self) -> Self {
        let mut ret = Self::ZERO;
        for r in 0..4 {
            for c in 0..4 {
                ret.data[c][r] = self.data[r][c];
            }
        }
        ret
    }

    pub fn submatrix(&self, r: usize, c: usize) -> Mat3 {
        let mut ret = Mat3::ZERO;

        for row in 0..3 {
            let rc = if row < r { 0 } else { 1 };
            for col in 0..3 {
                let cc = if col < c { 0 } else { 1 };
                ret.data[row][col] = self.data[row + rc][col + cc];
            }
        }
        ret
    }

    pub fn minor(&self, r: usize, c: usize) -> f32 {
        let sm = self.submatrix(r, c);
        sm.determinant()
    }

    pub fn cofactor(&self, r: usize, c: usize) -> f32 {
        if (r + c) % 2 == 1 {
            self.minor(r, c) * -1.0
        } else {
            self.minor(r, c)
        }
    }

    pub fn determinant(&self) -> f32 {
        self.data[0][0] * self.cofactor(0, 0)
            + self.data[0][1] * self.cofactor(0, 1)
            + self.data[0][2] * self.cofactor(0, 2)
            + self.data[0][3] * self.cofactor(0, 3)
    }

    pub fn inverse(&self) -> Result<Self, &'static str> {
        let d = self.determinant();
        if d == 0.0 {
            return Err("Matrix is not invertible");
        }
        let mut ret = Self::ZERO;
        for r in 0..4 {
            for c in 0..4 {
                ret.data[c][r] = self.cofactor(r, c) / d;
            }
        }
        Ok(ret)
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        let mut ret = Self::IDENTITY;
        ret.data[0][3] = x;
        ret.data[1][3] = y;
        ret.data[2][3] = z;
        ret
    }

    pub fn scaling(x: f32, y: f32, z: f32) -> Self {
        let mut ret = Self::IDENTITY;
        ret.data[0][0] = x;
        ret.data[1][1] = y;
        ret.data[2][2] = z;
        ret
    }

    pub fn rotation_x(r: f32) -> Self {
        let mut ret = Self::IDENTITY;
        ret.data[1][1] = r.cos();
        ret.data[1][2] = -r.sin();
        ret.data[2][1] = r.sin();
        ret.data[2][2] = r.cos();
        ret
    }

    pub fn rotation_y(r: f32) -> Self {
        let mut ret = Self::IDENTITY;
        ret.data[0][0] = r.cos();
        ret.data[2][0] = -r.sin();
        ret.data[0][2] = r.sin();
        ret.data[2][2] = r.cos();
        ret
    }

    pub fn rotation_z(r: f32) -> Self {
        let mut ret = Self::IDENTITY;
        ret.data[0][0] = r.cos();
        ret.data[0][1] = -r.sin();
        ret.data[1][0] = r.sin();
        ret.data[1][1] = r.cos();
        ret
    }

    pub fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Self {
        let mut ret = Self::IDENTITY;
        ret.data[0][1] = xy;
        ret.data[0][2] = xz;
        ret.data[1][0] = yx;
        ret.data[1][2] = yz;
        ret.data[2][0] = zx;
        ret.data[2][1] = zy;
        ret
    }

    pub fn view_transform(from: &Vec4, to: &Vec4, up: &Vec4) -> Self {
        let forward = (to - from).normalize();
        let n_up = up.normalize();
        let left = forward.cross(&n_up);
        let true_up = left.cross(&forward);
        let orientation = Self {
            data: [
                [left.x, left.y, left.z, 0.0],
                [true_up.x, true_up.y, true_up.z, 0.0],
                [-forward.x, -forward.y, -forward.z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        orientation * Self::translation(-from.x, -from.y, -from.z)
    }
}

impl Default for Mat4 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl From<[[f32; 4]; 4]> for Mat4 {
    fn from(data: [[f32; 4]; 4]) -> Self {
        Self { data }
    }
}

impl<T> Mul<T> for Mat4
where
    T: Into<Mat4>,
{
    type Output = Mat4;
    fn mul(self, rhs: T) -> Self::Output {
        let rhs: Mat4 = rhs.into();
        let mut ret = Mat4::ZERO;
        for r in 0..4 {
            for c in 0..4 {
                ret.data[r][c] = self.data[r][0] * rhs.data[0][c]
                    + self.data[r][1] * rhs.data[1][c]
                    + self.data[r][2] * rhs.data[2][c]
                    + self.data[r][3] * rhs.data[3][c];
            }
        }
        ret
    }
}

impl Mul<&Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: &Mat4) -> Self::Output {
        let mut ret = Mat4::ZERO;
        for r in 0..4 {
            for c in 0..4 {
                ret.data[r][c] = self.data[r][0] * rhs.data[0][c]
                    + self.data[r][1] * rhs.data[1][c]
                    + self.data[r][2] * rhs.data[2][c]
                    + self.data[r][3] * rhs.data[3][c];
            }
        }
        ret
    }
}

impl Mul<&Mat4> for &Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: &Mat4) -> Self::Output {
        let mut ret = Mat4::ZERO;
        for r in 0..4 {
            for c in 0..4 {
                ret.data[r][c] = self.data[r][0] * rhs.data[0][c]
                    + self.data[r][1] * rhs.data[1][c]
                    + self.data[r][2] * rhs.data[2][c]
                    + self.data[r][3] * rhs.data[3][c];
            }
        }
        ret
    }
}

impl Mul<Mat4> for &Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Self::Output {
        let mut ret = Mat4::ZERO;
        for r in 0..4 {
            for c in 0..4 {
                ret.data[r][c] = self.data[r][0] * rhs.data[0][c]
                    + self.data[r][1] * rhs.data[1][c]
                    + self.data[r][2] * rhs.data[2][c]
                    + self.data[r][3] * rhs.data[3][c];
            }
        }
        ret
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Self::Output {
        let mut ret: [[f32; 1]; 4] = [[0.0]; 4];
        for r in 0..4 {
            ret[r][0] = self.data[r][0] * rhs.x
                + self.data[r][1] * rhs.y
                + self.data[r][2] * rhs.z
                + self.data[r][3] * rhs.w;
        }
        Vec4::new(ret[0][0], ret[1][0], ret[2][0], ret[3][0])
    }
}

impl Mul<Vec4> for &Mat4 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Self::Output {
        let mut ret: [[f32; 1]; 4] = [[0.0]; 4];
        for r in 0..4 {
            ret[r][0] = self.data[r][0] * rhs.x
                + self.data[r][1] * rhs.y
                + self.data[r][2] * rhs.z
                + self.data[r][3] * rhs.w;
        }
        Vec4::new(ret[0][0], ret[1][0], ret[2][0], ret[3][0])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mat2 {
    pub data: [[f32; 2]; 2],
}

impl Mat2 {
    pub const ZERO: Self = Self {
        data: [[0.0; 2]; 2],
    };
    pub fn get(&self, r: usize, c: usize) -> f32 {
        self.data[r][c]
    }

    pub fn set(&mut self, r: usize, c: usize, val: f32) {
        self.data[r][c] = val;
    }

    pub fn determinant(&self) -> f32 {
        self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
    }
}

impl From<[[f32; 2]; 2]> for Mat2 {
    fn from(data: [[f32; 2]; 2]) -> Self {
        Self { data }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mat3 {
    pub data: [[f32; 3]; 3],
}

impl Mat3 {
    pub const ZERO: Self = Self {
        data: [[0.0; 3]; 3],
    };
    pub fn get(&self, r: usize, c: usize) -> f32 {
        self.data[r][c]
    }

    pub fn set(&mut self, r: usize, c: usize, val: f32) {
        self.data[r][c] = val;
    }

    pub fn submatrix(&self, r: usize, c: usize) -> Mat2 {
        let mut ret = Mat2::ZERO;

        for row in 0..2 {
            let rc = if row < r { 0 } else { 1 };
            for col in 0..2 {
                let cc = if col < c { 0 } else { 1 };
                ret.data[row][col] = self.data[row + rc][col + cc];
            }
        }
        ret
    }

    pub fn minor(&self, r: usize, c: usize) -> f32 {
        let sm = self.submatrix(r, c);
        sm.determinant()
    }

    pub fn cofactor(&self, r: usize, c: usize) -> f32 {
        if (r + c) % 2 == 1 {
            self.minor(r, c) * -1.0
        } else {
            self.minor(r, c)
        }
    }

    pub fn determinant(&self) -> f32 {
        self.data[0][0] * self.cofactor(0, 0)
            + self.data[0][1] * self.cofactor(0, 1)
            + self.data[0][2] * self.cofactor(0, 2)
    }
}

impl From<[[f32; 3]; 3]> for Mat3 {
    fn from(data: [[f32; 3]; 3]) -> Self {
        Self { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::EPSILON;
    use std::f32::consts::PI;

    #[test]
    fn new_mat4() {
        let data = [
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ];

        let m = Mat4::from(data);
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(0, 3), 4.0);
        assert_eq!(m.get(1, 0), 5.5);
        assert_eq!(m.get(1, 2), 7.5);
        assert_eq!(m.get(2, 2), 11.0);
        assert_eq!(m.get(3, 0), 13.5);
        assert_eq!(m.get(3, 2), 15.5);
    }

    #[test]
    fn new_mat2() {
        let data = [[-3.0, 5.0], [1.0, -2.0]];

        let m = Mat2::from(data);
        assert_eq!(m.get(0, 0), -3.0);
        assert_eq!(m.get(0, 1), 5.0);
        assert_eq!(m.get(1, 0), 1.0);
        assert_eq!(m.get(1, 1), -2.0);
    }

    #[test]
    fn new_mat3() {
        let data = [[1.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]];

        let m = Mat3::from(data);
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(0, 2), 3.0);
        assert_eq!(m.get(1, 0), 5.5);
        assert_eq!(m.get(1, 2), 7.5);
        assert_eq!(m.get(2, 2), 11.0);
    }

    #[test]
    fn eq_mat() {
        let data = [
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ];
        let m1 = Mat4::from(data.clone());
        let m2 = Mat4::from(data);
        assert_eq!(m1, m2);
        let data = [
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 12.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ];
        let m2 = Mat4::from(data);
        assert_ne!(m1, m2);
    }

    #[test]
    fn mat4_mul() {
        let data_a = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ];
        let data_b = [
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ];
        let data_exp = [
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ];
        let a = Mat4::from(data_a);
        let b = Mat4::from(data_b);
        let exp = Mat4::from(data_exp);
        assert_eq!(exp, a * b);
    }

    #[test]
    fn mat4_mul_vec4() {
        let m = Mat4::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let v = Vec4::new(1.0, 2.0, 3.0, 1.0);
        let exp = Vec4::new(18.0, 24.0, 33.0, 1.0);
        assert_eq!(exp, m * v);
        assert_eq!(v, Mat4::IDENTITY * v);
    }

    #[test]
    fn mat4_transpose() {
        let m = Mat4 {
            data: [
                [0.0, 9.0, 3.0, 0.0],
                [9.0, 8.0, 0.0, 8.0],
                [1.0, 8.0, 5.0, 3.0],
                [0.0, 0.0, 5.0, 8.0],
            ],
        };
        let exp = Mat4 {
            data: [
                [0.0, 9.0, 1.0, 0.0],
                [9.0, 8.0, 8.0, 0.0],
                [3.0, 0.0, 5.0, 5.0],
                [0.0, 8.0, 3.0, 8.0],
            ],
        };
        assert_eq!(exp, m.transpose());
        assert_eq!(Mat4::IDENTITY, Mat4::IDENTITY.transpose());
    }

    #[test]
    fn mat2_determinant() {
        let m = Mat2 {
            data: [[1.0, 5.0], [-3.0, 2.0]],
        };
        assert_eq!(17.0, m.determinant())
    }

    #[test]
    fn submatrix() {
        let m4 = Mat4 {
            data: [
                [-6.0, 1.0, 1.0, 6.0],
                [-8.0, 5.0, 8.0, 6.0],
                [-1.0, 0.0, 8.0, 2.0],
                [-7.0, 1.0, -1.0, 1.0],
            ],
        };
        let exp_m3_21 = Mat3 {
            data: [[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]],
        };
        let exp_m3_00 = Mat3 {
            data: [[5.0, 8.0, 6.0], [0.0, 8.0, 2.0], [1.0, -1.0, 1.0]],
        };
        let exp_m3_33 = Mat3 {
            data: [[-6.0, 1.0, 1.0], [-8.0, 5.0, 8.0], [-1.0, 0.0, 8.0]],
        };
        assert_eq!(exp_m3_00, m4.submatrix(0, 0));
        assert_eq!(exp_m3_21, m4.submatrix(2, 1));
        assert_eq!(exp_m3_33, m4.submatrix(3, 3));

        let m3 = Mat3 {
            data: [[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]],
        };
        let exp_m2_00 = Mat2 {
            data: [[2.0, 7.0], [6.0, -3.0]],
        };
        let exp_m2_02 = Mat2 {
            data: [[-3.0, 2.0], [0.0, 6.0]],
        };
        let exp_m2_11 = Mat2 {
            data: [[1.0, 0.0], [0.0, -3.0]],
        };
        let exp_m2_10 = Mat2 {
            data: [[5.0, 0.0], [6.0, -3.0]],
        };
        let exp_m2_21 = Mat2 {
            data: [[1.0, 0.0], [-3.0, 7.0]],
        };
        assert_eq!(exp_m2_00, m3.submatrix(0, 0));
        assert_eq!(exp_m2_02, m3.submatrix(0, 2));
        assert_eq!(exp_m2_11, m3.submatrix(1, 1));
        assert_eq!(exp_m2_10, m3.submatrix(1, 0));
        assert_eq!(exp_m2_21, m3.submatrix(2, 1));
    }

    #[test]
    fn mat3_minor() {
        let m = Mat3 {
            data: [[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]],
        };
        let sm = m.submatrix(1, 0);
        assert_eq!(25.0, sm.determinant());
        assert_eq!(25.0, m.minor(1, 0));
    }

    #[test]
    fn mat3_cofactor() {
        let m = Mat3 {
            data: [[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]],
        };
        assert_eq!(-12.0, m.minor(0, 0));
        assert_eq!(-12.0, m.cofactor(0, 0));
        assert_eq!(25.0, m.minor(1, 0));
        assert_eq!(-25.0, m.cofactor(1, 0));
    }

    #[test]
    fn mat3_determinant() {
        let m = Mat3 {
            data: [[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]],
        };
        assert_eq!(56.0, m.cofactor(0, 0));
        assert_eq!(12.0, m.cofactor(0, 1));
        assert_eq!(-46.0, m.cofactor(0, 2));
        assert_eq!(-196.0, m.determinant());
    }

    #[test]
    fn mat4_determinant() {
        let m = Mat4 {
            data: [
                [-2.0, -8.0, 3.0, 5.0],
                [-3.0, 1.0, 7.0, 3.0],
                [1.0, 2.0, -9.0, 6.0],
                [-6.0, 7.0, 7.0, -9.0],
            ],
        };
        assert_eq!(690.0, m.cofactor(0, 0));
        assert_eq!(447.0, m.cofactor(0, 1));
        assert_eq!(210.0, m.cofactor(0, 2));
        assert_eq!(51.0, m.cofactor(0, 3));
        assert_eq!(-4071.0, m.determinant());
    }

    #[test]
    fn mat4_invertible() {
        // if determinant != 0 thne inverible
        let m_inv = Mat4 {
            data: [
                [6.0, 4.0, 4.0, 4.0],
                [5.0, 5.0, 7.0, 6.0],
                [4.0, -9.0, 3.0, -7.0],
                [9.0, 1.0, 7.0, -6.0],
            ],
        };
        assert_eq!(-2120.0, m_inv.determinant());

        let m_not_inv = Mat4 {
            data: [
                [-4.0, 2.0, -2.0, -3.0],
                [9.0, 6.0, 2.0, 6.0],
                [0.0, -5.0, 1.0, -5.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        };
        assert_eq!(0.0, m_not_inv.determinant());
        assert!(m_not_inv.inverse().is_err());
    }

    #[test]
    fn mat4_inverse() {
        let m = Mat4 {
            data: [
                [-5.0, 2.0, 6.0, -8.0],
                [1.0, -5.0, 1.0, 8.0],
                [7.0, 7.0, -6.0, -7.0],
                [1.0, -3.0, 7.0, 4.0],
            ],
        };
        let m_inverse = m.inverse().unwrap();
        assert_eq!(532.0, m.determinant());
        assert_eq!(-160.0, m.cofactor(2, 3));
        assert_eq!(-160.0 / 532.0, m_inverse.data[3][2]);
        assert_eq!(105.0, m.cofactor(3, 2));
        assert_eq!(105.0 / 532.0, m_inverse.data[2][3]);
        let exp = Mat4 {
            data: [
                [0.21805, 0.45113, 0.24060, -0.04511],
                [-0.80827, -1.45677, -0.44361, 0.52068],
                [-0.07895, -0.22368, -0.05263, 0.19737],
                [-0.52256, -0.81391, -0.30075, 0.30639],
            ],
        };
        for r in 0..4 {
            for c in 0..4 {
                assert!(
                    (exp.data[r][c] - m_inverse.data[r][c]).abs() < EPSILON,
                    "Differs at [{}][{}]",
                    r,
                    c
                );
            }
        }

        let m = Mat4 {
            data: [
                [8.0, -5.0, 9.0, 2.0],
                [7.0, 5.0, 6.0, 1.0],
                [-6.0, 0.0, 9.0, 6.0],
                [-3.0, 0.0, -9.0, -4.0],
            ],
        };
        let m_inverse = m.inverse().unwrap();
        let exp = Mat4 {
            data: [
                [-0.15385, -0.15385, -0.28205, -0.53846],
                [-0.07692, 0.12308, 0.02564, 0.03077],
                [0.35897, 0.35897, 0.43590, 0.92308],
                [-0.69231, -0.69231, -0.76923, -1.92308],
            ],
        };
        for r in 0..4 {
            for c in 0..4 {
                assert!(
                    (exp.data[r][c] - m_inverse.data[r][c]).abs() < EPSILON,
                    "Differs at [{}][{}]",
                    r,
                    c
                );
            }
        }

        let m = Mat4 {
            data: [
                [9.0, 3.0, 0.0, 9.0],
                [-5.0, -2.0, -6.0, -3.0],
                [-4.0, 9.0, 6.0, 4.0],
                [-7.0, 6.0, 6.0, 2.0],
            ],
        };
        let m_inverse = m.inverse().unwrap();
        let exp = Mat4 {
            data: [
                [-0.04074, -0.07778, 0.14444, -0.22222],
                [-0.07778, 0.03333, 0.36667, -0.33333],
                [-0.02901, -0.14630, -0.10926, 0.12963],
                [0.17778, 0.06667, -0.26667, 0.33333],
            ],
        };
        for r in 0..4 {
            for c in 0..4 {
                assert!(
                    (exp.data[r][c] - m_inverse.data[r][c]).abs() < EPSILON,
                    "Differs at [{}][{}]",
                    r,
                    c
                );
            }
        }

        // Inversion proof
        let a = Mat4 {
            data: [
                [3.0, -9.0, 7.0, 3.0],
                [3.0, -8.0, 2.0, -9.0],
                [-4.0, 4.0, 4.0, 1.0],
                [-6.0, 5.0, -1.0, 1.0],
            ],
        };
        let b = Mat4 {
            data: [
                [8.0, 2.0, 2.0, 2.0],
                [3.0, -1.0, 7.0, 0.0],
                [7.0, 0.0, 5.0, 4.0],
                [6.0, -2.0, 0.0, 5.0],
            ],
        };
        let c = &a * &b;
        let b_inverse = b.inverse().unwrap();
        let ret_a = c * b_inverse;
        for r in 0..4 {
            for c in 0..4 {
                assert!(
                    (a.data[r][c] - ret_a.data[r][c]).abs() < EPSILON,
                    "Differs at [{}][{}]",
                    r,
                    c
                );
            }
        }
    }

    #[test]
    fn translation_mul() {
        let transform = Mat4::translation(5.0, -3.0, 2.0);
        let p = Vec4::new_point(-3.0, 4.0, 5.0);
        let inverse_transform = transform.inverse().unwrap();
        assert_eq!(Vec4::new_point(2.0, 1.0, 7.0), &transform * p);
        assert_eq!(Vec4::new_point(-8.0, 7.0, 3.0), inverse_transform * p);
        let v = Vec4::new_vec(-3.0, 4.0, 5.0);
        assert_eq!(v, &transform * v);
    }

    #[test]
    fn scaling() {
        let t = Mat4::scaling(2.0, 3.0, 4.0);
        let p = Vec4::new_point(-4.0, 6.0, 8.0);
        assert_eq!(Vec4::new_point(-8.0, 18.0, 32.0), &t * p);
        let v = Vec4::new_vec(-4.0, 6.0, 8.0);
        assert_eq!(Vec4::new_vec(-8.0, 18.0, 32.0), &t * v);
        assert_eq!(Vec4::new_vec(-2.0, 2.0, 2.0), t.inverse().unwrap() * v);

        // Reflection - mirror x
        let t = Mat4::scaling(-1.0, 1.0, 1.0);
        let p = Vec4::new_point(2.0, 3.0, 4.0);
        assert_eq!(Vec4::new_point(-2.0, 3.0, 4.0), &t * p);
    }

    #[test]
    fn rotation() {
        // x
        let p = Vec4::new_point(0.0, 1.0, 0.0);
        let half_quarter = Mat4::rotation_x(PI / 4.0);
        let full_quarter = Mat4::rotation_x(PI / 2.0);
        assert_eq!(
            Vec4::new_point(0.0, 2f32.sqrt() / 2.0, 2f32.sqrt() / 2.0),
            half_quarter * p
        );
        let exp = Vec4::new_point(0.0, 0.0, 1.0);
        assert_eq!(exp, full_quarter * p);

        // y
        let p = Vec4::new_point(0.0, 0.0, 1.0);
        let half_quarter = Mat4::rotation_y(PI / 4.0);
        let full_quarter = Mat4::rotation_y(PI / 2.0);
        assert_eq!(
            Vec4::new_point(2f32.sqrt() / 2.0, 0.0, 2f32.sqrt() / 2.0),
            half_quarter * p
        );
        let exp = Vec4::new_point(1.0, 0.0, 0.0);
        assert_eq!(exp, full_quarter * p);

        // z
        let p = Vec4::new_point(0.0, 1.0, 0.0);
        let half_quarter = Mat4::rotation_z(PI / 4.0);
        let full_quarter = Mat4::rotation_z(PI / 2.0);
        assert_eq!(
            Vec4::new_point(-2f32.sqrt() / 2.0, 2f32.sqrt() / 2.0, 0.0),
            half_quarter * p
        );
        let exp = Vec4::new_point(-1.0, 0.0, 0.0);
        assert_eq!(exp, full_quarter * p);
    }

    #[test]
    fn shearing() {
        let p = Vec4::new_point(2.0, 3.0, 4.0);

        let t = Mat4::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(Vec4::new_point(5.0, 3.0, 4.0), t * p);

        let t = Mat4::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(Vec4::new_point(6.0, 3.0, 4.0), t * p);

        let t = Mat4::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        assert_eq!(Vec4::new_point(2.0, 5.0, 4.0), t * p);

        let t = Mat4::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        assert_eq!(Vec4::new_point(2.0, 7.0, 4.0), t * p);

        let t = Mat4::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        assert_eq!(Vec4::new_point(2.0, 3.0, 6.0), t * p);

        let t = Mat4::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        assert_eq!(Vec4::new_point(2.0, 3.0, 7.0), t * p);
    }

    #[test]
    fn combo_transform() {
        let p = Vec4::new_point(1.0, 0.0, 1.0);
        let rot = Mat4::rotation_x(PI / 2.0);
        let scale = Mat4::scaling(5.0, 5.0, 5.0);
        let t = Mat4::translation(10.0, 5.0, 7.0);

        let p2 = &rot * p;
        assert_eq!(p2, Vec4::new_point(1.0, -1.0, 0.0));

        let p3 = &scale * p2;
        assert_eq!(p3, Vec4::new_point(5.0, -5.0, 0.0));

        let p4 = &t * p3;
        assert_eq!(p4, Vec4::new_point(15.0, 0.0, 7.0));

        let transform = t * scale * rot;
        let p5 = transform * p;
        assert_eq!(p5, Vec4::new_point(15.0, 0.0, 7.0));
    }

    fn approx_eq(a: &Mat4, b: &Mat4) {
        for r in 0..4 {
            for c in 0..4 {
                let n = (a.data[r][c] - b.data[r][c]).abs();
                assert!(
                    n < EPSILON,
                    "Matrices differ at [{}][{}]: {:.8} != {:.8}",
                    r,
                    c,
                    a.data[r][c],
                    b.data[r][c]
                );
            }
        }
    }

    #[test]
    fn view_transform() {
        let from = Vec4::POINT_ZERO;
        let to = Vec4::new_point(0.0, 0.0, -1.0);
        let up = Vec4::VEC_Y_ONE;
        let t = Mat4::view_transform(&from, &to, &up);
        assert_eq!(Mat4::IDENTITY, t);

        let from = Vec4::POINT_ZERO;
        let to = Vec4::new_point(0.0, 0.0, 1.0);
        let up = Vec4::VEC_Y_ONE;
        let t = Mat4::view_transform(&from, &to, &up);
        assert_eq!(Mat4::scaling(-1.0, 1.0, -1.0), t);

        let from = Vec4::new_point(0.0, 0.0, 8.0);
        let to = Vec4::POINT_ZERO;
        let up = Vec4::VEC_Y_ONE;
        let t = Mat4::view_transform(&from, &to, &up);
        assert_eq!(Mat4::translation(0.0, 0.0, -8.0), t);

        let from = Vec4::new_point(1.0, 3.0, 2.0);
        let to = Vec4::new_point(4.0, -2.0, 8.0);
        let up = Vec4::new_vec(1.0, 1.0, 0.0);
        let t = Mat4::view_transform(&from, &to, &up);
        let exp = Mat4 {
            data: [
                [-0.50709, 0.50709, 0.67612, -2.36643],
                [0.76772, 0.60609, 0.12122, -2.82843],
                [-0.35857, 0.59761, -0.71714, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        approx_eq(&t, &exp);
    }
}
