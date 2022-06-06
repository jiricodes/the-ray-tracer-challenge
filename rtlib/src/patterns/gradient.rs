use crate::color::Color;
use crate::math::matrix::Mat4;
use crate::math::vec4::Vec4;
use crate::patterns::{BoxPattern, Pattern};

use std::any::Any;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub struct GradientPattern {
    start_color: Color,
    end_color: Color,
    transform: Mat4,
    inverse_transform: Mat4,
}

impl GradientPattern {
    pub fn new(start_color: Color, end_color: Color, transform: Option<Mat4>) -> Self {
        Self {
            start_color,
            end_color,
            transform: transform.unwrap_or_default(),
            inverse_transform: transform
                .unwrap_or_default()
                .inverse()
                .expect("Pattern transform"),
        }
    }

    pub fn new_boxed(start_color: Color, end_color: Color, transform: Option<Mat4>) -> BoxPattern {
        Box::new(Self::new(start_color, end_color, transform))
    }

    pub fn default_boxed() -> BoxPattern {
        Box::new(Self::default())
    }
}

impl Pattern for GradientPattern {
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
        let x = local_point.x - local_point.x.floor();
        self.start_color + (self.end_color - self.start_color) * x
    }
}

impl Default for GradientPattern {
    fn default() -> Self {
        Self {
            start_color: Color::WHITE,
            end_color: Color::BLACK,
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
        let pattern = GradientPattern::default();
        assert_eq!(pattern.start_color, Color::WHITE);
        assert_eq!(pattern.end_color, Color::BLACK);
    }

    #[test]
    fn pattern_at() {
        let pattern = GradientPattern::default();
        assert_eq!(pattern.local_pattern_at(Vec4::POINT_ZERO), Color::WHITE);
        assert_eq!(
            pattern.local_pattern_at(Vec4::point(0.25, 0.0, 0.0)),
            Color::rgb(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.local_pattern_at(Vec4::point(0.5, 0.0, 0.0)),
            Color::rgb(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.local_pattern_at(Vec4::point(0.75, 0.0, 0.0)),
            Color::rgb(0.25, 0.25, 0.25)
        );
    }
}
