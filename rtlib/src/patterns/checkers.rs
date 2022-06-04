use crate::color::Color;
use crate::math::matrix::Mat4;
use crate::math::vec4::Vec4;
use crate::patterns::{BoxPattern, Pattern};

use std::any::Any;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub struct CheckersPattern {
    color_a: Color,
    color_b: Color,
    transform: Mat4,
    inverse_transform: Mat4,
}

impl CheckersPattern {
    pub fn new(color_a: Color, color_b: Color, transform: Option<Mat4>) -> Self {
        Self {
            color_a,
            color_b,
            transform: transform.unwrap_or_default(),
            inverse_transform: transform
                .unwrap_or_default()
                .inverse()
                .expect("Pattern transform"),
        }
    }

    pub fn new_boxed(color_a: Color, color_b: Color, transform: Option<Mat4>) -> BoxPattern {
        Box::new(Self::new(color_a, color_b, transform))
    }

    pub fn default_boxed() -> BoxPattern {
        Box::new(Self::default())
    }
}

impl Pattern for CheckersPattern {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn box_clone(&self) -> BoxPattern {
        Box::new((*self).clone())
    }
    fn box_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn transform(&mut self, m: &Mat4) {
        self.transform = m * self.transform;
    }
    fn set_transform(&mut self, transformation: Mat4) {
        self.transform = transformation;
        self.inverse_transform = transformation.inverse().expect("Pattern transform");
    }
    fn transformation(&self) -> &Mat4 {
        &self.transform
    }
    fn inverse_transformation(&self) -> &Mat4 {
        &self.inverse_transform
    }

    fn local_pattern_at(&self, local_point: Vec4) -> Color {
        let i = (local_point.x.floor() + local_point.y.floor() + local_point.z.floor()).abs()
            as usize
            % 2;
        if i == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }
}

impl Default for CheckersPattern {
    fn default() -> Self {
        Self {
            color_a: Color::WHITE,
            color_b: Color::BLACK,
            transform: Mat4::default(),
            inverse_transform: Mat4::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::shapes::Sphere;

    #[test]
    fn basic() {
        let pattern = CheckersPattern::default();
        assert_eq!(pattern.color_a, Color::WHITE);
        assert_eq!(pattern.color_b, Color::BLACK);
    }

    #[test]
    fn pattern_at_x() {
        let pattern = CheckersPattern::default();
        assert_eq!(pattern.local_pattern_at(Vec4::POINT_ZERO), Color::WHITE);
        assert_eq!(
            pattern.local_pattern_at(Vec4::new_point(0.99, 0.0, 0.0)),
            Color::WHITE
        );
        assert_eq!(
            pattern.local_pattern_at(Vec4::new_point(1.01, 0.0, 0.0)),
            Color::BLACK
        );
    }

    #[test]
    fn pattern_at_y() {
        let pattern = CheckersPattern::default();
        assert_eq!(pattern.local_pattern_at(Vec4::POINT_ZERO), Color::WHITE);
        assert_eq!(
            pattern.local_pattern_at(Vec4::new_point(0.0, 0.99, 0.0)),
            Color::WHITE
        );
        assert_eq!(
            pattern.local_pattern_at(Vec4::new_point(0.0, 1.01, 0.0)),
            Color::BLACK
        );
    }

    #[test]
    fn pattern_at_z() {
        let pattern = CheckersPattern::default();
        assert_eq!(pattern.local_pattern_at(Vec4::POINT_ZERO), Color::WHITE);
        assert_eq!(
            pattern.local_pattern_at(Vec4::new_point(0.0, 0.0, 0.99)),
            Color::WHITE
        );
        assert_eq!(
            pattern.local_pattern_at(Vec4::new_point(0.0, 0.0, 1.01)),
            Color::BLACK
        );
    }

    #[test]
    fn paternion_at_xz() {
        let pattern = CheckersPattern::default();
        assert_eq!(pattern.local_pattern_at(Vec4::POINT_ZERO), Color::WHITE);
        assert_eq!(
            pattern.local_pattern_at(Vec4::new_point(0.99, 0.0, 0.99)),
            Color::WHITE
        );
        assert_eq!(
            pattern.local_pattern_at(Vec4::new_point(0.99, 0.0, 1.09)),
            Color::BLACK
        );
        assert_eq!(
            pattern.local_pattern_at(Vec4::new_point(1.09, 0.0, 1.09)),
            Color::WHITE
        );
    }
}
