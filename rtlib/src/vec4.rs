use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    /// Our minimum error tollerance
    const _EPSILON: f32 = 0.00001;
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn new_point(x: f32, y: f32, z: f32) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: 1.0,
        }
    }

    pub fn new_vec(x: f32, y: f32, z: f32) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: 0.0,
        }
    }
}

impl<T> Add<T> for Vec4
where
    T: Into<Vec4>,
{
    type Output = Self;
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
    type Output = Self;
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
}
