use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    /// Our minimum error tollerance
    const _EPSILON: f32 = 0.00001;
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };
    pub const POINT_ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn new_point(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub fn new_vec(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let m = self.magnitude();
        self / m
    }

    fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.x.abs(),
            z: self.z.abs(),
            w: self.w.abs(),
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: &Self) -> Self {
        if self.w != 0.0 {
            panic!("Attempting cross product on non-vectors (w != 0f32)");
        }
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: 0.0,
        }
    }
}

impl PartialEq<Vec4> for Vec4 {
    fn eq(&self, other: &Vec4) -> bool {
        let a = (self - other).abs();
        a.x < Vec4::_EPSILON && a.y < Vec4::_EPSILON && a.z < Vec4::_EPSILON && a.w < Vec4::_EPSILON
    }
}

impl<T> Add<T> for Vec4
where
    T: Into<Vec4>,
{
    type Output = Vec4;
    fn add(self, rhs: T) -> Self::Output {
        let rhs: Vec4 = rhs.into();
        Vec4::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl<T> Add<T> for &Vec4
where
    T: Into<Vec4>,
{
    type Output = Vec4;
    fn add(self, rhs: T) -> Self::Output {
        let rhs: Vec4 = rhs.into();
        Vec4::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl<T> Sub<T> for Vec4
where
    T: Into<Vec4>,
{
    type Output = Vec4;
    fn sub(self, rhs: T) -> Self::Output {
        let rhs: Vec4 = rhs.into();
        Vec4::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl<T> Sub<T> for &Vec4
where
    T: Into<Vec4>,
{
    type Output = Vec4;
    fn sub(self, rhs: T) -> Self::Output {
        let rhs: Vec4 = rhs.into();
        Vec4::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl Sub<&Vec4> for &Vec4 {
    type Output = Vec4;
    fn sub(self, rhs: &Vec4) -> Self::Output {
        Vec4::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;
    fn mul(self, rhs: f32) -> Self::Output {
        let m: f32 = rhs.into();
        Self::Output {
            x: self.x * m,
            y: self.y * m,
            z: self.z * m,
            w: self.w * m,
        }
    }
}

impl<T> Mul<T> for &Vec4
where
    T: Into<f32>,
{
    type Output = Vec4;
    fn mul(self, rhs: T) -> Self::Output {
        let m: f32 = rhs.into();
        Self::Output {
            x: self.x * m,
            y: self.y * m,
            z: self.z * m,
            w: self.w * m,
        }
    }
}

impl<T> Div<T> for Vec4
where
    T: Into<f32>,
{
    type Output = Vec4;
    fn div(self, rhs: T) -> Self::Output {
        let m: f32 = rhs.into();
        if m == 0.0 {
            panic!("Cannot divide by zero-valued!");
        }
        Self::Output {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
            w: self.w / m,
        }
    }
}

impl<T> Div<T> for &Vec4
where
    T: Into<f32>,
{
    type Output = Vec4;
    fn div(self, rhs: T) -> Self::Output {
        let m: f32 = rhs.into();
        if m == 0.0 {
            panic!("Cannot divide by zero-valued!");
        }
        Self::Output {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
            w: self.w / m,
        }
    }
}

impl Neg for Vec4 {
    type Output = Vec4;
    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl Neg for &Vec4 {
    type Output = Vec4;
    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn point_and_vector() {
        let p = Vec4::new_point(4.3, -4.2, 3.1);
        assert_eq!(p.x, 4.3);
        assert_eq!(p.y, -4.2);
        assert_eq!(p.z, 3.1);
        assert_eq!(p.w, 1.0);
        let mut v = Vec4::new_vec(4.3, -4.2, 3.1);
        assert_eq!(v.x, 4.3);
        assert_eq!(v.y, -4.2);
        assert_eq!(v.z, 3.1);
        assert_eq!(v.w, 0.0);
        assert_ne!(v, p);

        // minimum error check
        v.w += Vec4::_EPSILON;
        assert_ne!(v, p, "Minimum error check");
    }

    #[test]
    fn adding() {
        let a = Vec4::new(3.0, -2.0, 5.0, 1.0);
        let b = Vec4::new(-2.0, 3.0, 1.0, 0.0);
        assert_eq!(Vec4::new(1.0, 1.0, 6.0, 1.0), a + b);
    }

    #[test]
    fn subbing() {
        // Point - Point is vector
        let a = Vec4::new_point(3.0, 2.0, 1.0);
        let b = Vec4::new_point(5.0, 6.0, 7.0);
        assert_eq!(Vec4::new_vec(-2.0, -4.0, -6.0), a - b);

        // Point - vec is point
        let a = Vec4::new_point(3.0, 2.0, 1.0);
        let b = Vec4::new_vec(5.0, 6.0, 7.0);
        assert_eq!(Vec4::new_point(-2.0, -4.0, -6.0), a - b);

        // vec - vec is vec
        let a = Vec4::new_vec(3.0, 2.0, 1.0);
        let b = Vec4::new_vec(5.0, 6.0, 7.0);
        assert_eq!(Vec4::new_vec(-2.0, -4.0, -6.0), a - b);
    }

    #[test]
    fn multiply() {
        let z = Vec4::ZERO;
        let v: Vec4 = Vec4::new_vec(1.0, -2.0, 3.0);
        let e = Vec4::new_vec(-1.0, 2.0, -3.0);
        assert_eq!(e, z - v);
        assert_eq!(e, -v);
        let x = Vec4::new(-2.0, -4.0, -6.0, -8.0);
        assert_eq!(Vec4::new(2.0, 4.0, 6.0, 8.0), -x);
        let x = Vec4::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(Vec4::new(3.5, -7.0, 10.5, -14.0), x * 3.5);
        let x = Vec4::new(2.0, -4.0, 6.0, -8.0);
        assert_eq!(Vec4::new(1.0, -2.0, 3.0, -4.0), x * 0.5);
    }

    #[test]
    fn div() {
        let x = Vec4::new(2.0, -4.0, 6.0, -8.0);
        assert_eq!(Vec4::new(1.0, -2.0, 3.0, -4.0), x / 2.0);
    }

    #[test]
    fn magnitude() {
        let v = Vec4::new_vec(1.0, 0.0, 0.0);
        assert_eq!(1.0, v.magnitude());
        let v = Vec4::new_vec(0.0, 1.0, 0.0);
        assert_eq!(1.0, v.magnitude());
        let v = Vec4::new_vec(0.0, 0.0, 1.0);
        assert_eq!(1.0, v.magnitude());
        let v = Vec4::new_vec(1.0, 2.0, 3.0);
        assert_eq!(14.0f32.sqrt(), v.magnitude());
        let v = Vec4::new_vec(-1.0, -2.0, -3.0);
        assert_eq!(14.0f32.sqrt(), v.magnitude());
    }

    #[test]
    fn normalize() {
        let v = Vec4::new_vec(4.0, 0.0, 0.0);
        let exp = Vec4::new_vec(1.0, 0.0, 0.0);
        assert_eq!(exp, v.normalize());

        let v = Vec4::new_vec(1.0, 2.0, 3.0);
        let exp = Vec4::new_vec(1.0 / 14f32.sqrt(), 2.0 / 14f32.sqrt(), 3.0 / 14f32.sqrt());
        assert_eq!(exp, v.normalize());

        let v = Vec4::new_vec(1.0, 2.0, 3.0);
        let v = v.normalize();
        assert!((1.0 - v.magnitude()).abs() < Vec4::_EPSILON);
    }

    #[test]
    fn dot() {
        let a = Vec4::new_vec(1.0, 2.0, 3.0);
        let b = Vec4::new_vec(2.0, 3.0, 4.0);
        assert_eq!(20.0, a.dot(&b));
    }

    #[test]
    fn cross() {
        let a = Vec4::new_vec(1.0, 2.0, 3.0);
        let b = Vec4::new_vec(2.0, 3.0, 4.0);
        assert_eq!(Vec4::new_vec(-1.0, 2.0, -1.0), a.cross(&b));
        assert_eq!(Vec4::new_vec(1.0, -2.0, 1.0), b.cross(&a));
    }
}
