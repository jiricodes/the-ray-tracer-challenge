use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    /// Our minimum error tollerance
    const _EPSILON: f32 = 0.00001;
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };

    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }

    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: r,
            g: g,
            b: b,
            a: 0.0,
        }
    }
}

impl<T> Add<T> for Color
where
    T: Into<Color>,
{
    type Output = Color;
    fn add(self, rhs: T) -> Self::Output {
        let rhs: Color = rhs.into();
        Color::rgba(
            self.r + rhs.r,
            self.g + rhs.g,
            self.b + rhs.b,
            self.a + rhs.a,
        )
    }
}

impl<T> Add<T> for &Color
where
    T: Into<Color>,
{
    type Output = Color;
    fn add(self, rhs: T) -> Self::Output {
        let rhs: Color = rhs.into();
        Color::rgba(
            self.r + rhs.r,
            self.g + rhs.g,
            self.b + rhs.b,
            self.a + rhs.a,
        )
    }
}

impl<T> Sub<T> for Color
where
    T: Into<Color>,
{
    type Output = Color;
    fn sub(self, rhs: T) -> Self::Output {
        let rhs: Color = rhs.into();
        Color::rgba(
            self.r - rhs.r,
            self.g - rhs.g,
            self.b - rhs.b,
            self.a - rhs.a,
        )
    }
}

impl<T> Sub<T> for &Color
where
    T: Into<Color>,
{
    type Output = Color;
    fn sub(self, rhs: T) -> Self::Output {
        let rhs: Color = rhs.into();
        Color::rgba(
            self.r - rhs.r,
            self.g - rhs.g,
            self.b - rhs.b,
            self.a - rhs.a,
        )
    }
}

impl<T> Mul<T> for Color
where
    T: Into<f32>,
{
    type Output = Color;
    fn mul(self, rhs: T) -> Self::Output {
        let m: f32 = rhs.into();
        Self::Output {
            r: self.r * m,
            g: self.g * m,
            b: self.b * m,
            a: self.a * m,
        }
    }
}

impl<T> Mul<T> for &Color
where
    T: Into<f32>,
{
    type Output = Color;
    fn mul(self, rhs: T) -> Self::Output {
        let m: f32 = rhs.into();
        Self::Output {
            r: self.r * m,
            g: self.g * m,
            b: self.b * m,
            a: self.a * m,
        }
    }
}

impl<T> Div<T> for Color
where
    T: Into<f32>,
{
    type Output = Color;
    fn div(self, rhs: T) -> Self::Output {
        let m: f32 = rhs.into();
        if m == 0.0 {
            panic!("Cannot divide by zero-valued!");
        }
        Self::Output {
            r: self.r / m,
            g: self.g / m,
            b: self.b / m,
            a: self.a / m,
        }
    }
}

impl<T> Div<T> for &Color
where
    T: Into<f32>,
{
    type Output = Color;
    fn div(self, rhs: T) -> Self::Output {
        let m: f32 = rhs.into();
        if m == 0.0 {
            panic!("Cannot divide by zero-valued!");
        }
        Self::Output {
            r: self.r / m,
            g: self.g / m,
            b: self.b / m,
            a: self.a / m,
        }
    }
}

impl Neg for Color {
    type Output = Color;
    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl Neg for &Color {
    type Output = Color;
    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn point_and_vector() {
        let p = Color::rgb(4.3, -4.2, 3.1);
        assert_eq!(p.r, 4.3);
        assert_eq!(p.g, -4.2);
        assert_eq!(p.b, 3.1);
        assert_eq!(p.a, 0.0);
        let mut v = Color::rgba(4.3, -4.2, 3.1, 0.7);
        assert_eq!(v.r, 4.3);
        assert_eq!(v.g, -4.2);
        assert_eq!(v.b, 3.1);
        assert_eq!(v.a, 0.7);
        assert_ne!(v, p);

        // minimum error check
        v.a += Color::_EPSILON;
        assert_ne!(v, p, "Minimum error check");
    }

    #[test]
    fn adding() {
        let a = Color::rgba(3.0, -2.0, 5.0, 1.0);
        let b = Color::rgba(-2.0, 3.0, 1.0, 0.0);
        assert_eq!(Color::rgba(1.0, 1.0, 6.0, 1.0), a + b);
    }

    #[test]
    fn subbing() {
        // Point - Point is vector
        let a = Color::rgb(3.0, 2.0, 1.0);
        let b = Color::rgb(5.0, 6.0, 7.0);
        assert_eq!(Color::rgb(-2.0, -4.0, -6.0), a - b);

        // Point - vec is point
        let a = Color::rgb(3.0, 2.0, 1.0);
        let b = Color::rgb(5.0, 6.0, 7.0);
        assert_eq!(Color::rgb(-2.0, -4.0, -6.0), a - b);

        // vec - vec is vec
        let a = Color::rgb(3.0, 2.0, 1.0);
        let b = Color::rgb(5.0, 6.0, 7.0);
        assert_eq!(Color::rgb(-2.0, -4.0, -6.0), a - b);
    }

    #[test]
    fn multiply() {
        let z = Color::BLACK;
        let v: Color = Color::rgb(1.0, -2.0, 3.0);
        let e = Color::rgb(-1.0, 2.0, -3.0);
        assert_eq!(e, z - v);
        assert_eq!(e, -v);
        let x = Color::rgba(-2.0, -4.0, -6.0, -8.0);
        assert_eq!(Color::rgba(2.0, 4.0, 6.0, 8.0), -x);
        let x = Color::rgba(1.0, -2.0, 3.0, -4.0);
        assert_eq!(Color::rgba(3.5, -7.0, 10.5, -14.0), x * 3.5);
        let x = Color::rgba(2.0, -4.0, 6.0, -8.0);
        assert_eq!(Color::rgba(1.0, -2.0, 3.0, -4.0), x * 0.5);
    }

    #[test]
    fn div() {
        let x = Color::rgba(2.0, -4.0, 6.0, -8.0);
        assert_eq!(Color::rgba(1.0, -2.0, 3.0, -4.0), x / 2.0);
    }
}
