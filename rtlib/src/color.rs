use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    /// Our minimum error tollerance
    const _EPSILON: f32 = 0.00001;
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
    };
    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
    };
    pub const BLUE: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
    };
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };

    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn hadamard_product(&self, other: &Color) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
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
        Color::rgb(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl<T> Add<T> for &Color
where
    T: Into<Color>,
{
    type Output = Color;
    fn add(self, rhs: T) -> Self::Output {
        let rhs: Color = rhs.into();
        Color::rgb(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl<T> Sub<T> for Color
where
    T: Into<Color>,
{
    type Output = Color;
    fn sub(self, rhs: T) -> Self::Output {
        let rhs: Color = rhs.into();
        Color::rgb(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl<T> Sub<T> for &Color
where
    T: Into<Color>,
{
    type Output = Color;
    fn sub(self, rhs: T) -> Self::Output {
        let rhs: Color = rhs.into();
        Color::rgb(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn point_and_vector() {
        let mut p = Color::rgb(4.3, -4.2, 3.1);
        assert_eq!(p.r, 4.3);
        assert_eq!(p.g, -4.2);
        assert_eq!(p.b, 3.1);

        // minimum error check
        let v = p.clone();
        p.b += Color::_EPSILON;
        assert_ne!(v, p, "Minimum error check");
    }

    #[test]
    fn adding() {
        let a = Color::rgb(3.0, -2.0, 5.0);
        let b = Color::rgb(-2.0, 3.0, 1.0);
        assert_eq!(Color::rgb(1.0, 1.0, 6.0), a + b);
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
        let x = Color::rgb(1.0, -2.0, 3.0);
        assert_eq!(Color::rgb(3.5, -7.0, 10.5), x * 3.5);
        let x = Color::rgb(2.0, -4.0, 6.0);
        assert_eq!(Color::rgb(1.0, -2.0, 3.0), x * 0.5);
    }

    #[test]
    fn div() {
        let x = Color::rgb(2.0, -4.0, 6.0);
        assert_eq!(Color::rgb(1.0, -2.0, 3.0), x / 2.0);
    }
}
