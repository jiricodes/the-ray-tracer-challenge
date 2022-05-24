use crate::vec4::Vec4;
use std::ops::Mul;

#[derive(Debug, Clone, PartialEq)]
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
}

impl From<[[f32; 3]; 3]> for Mat3 {
    fn from(data: [[f32; 3]; 3]) -> Self {
        Self { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
