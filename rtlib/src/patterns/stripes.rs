use crate::color::Color;
use crate::math::matrix::Mat4;
use crate::math::vec4::Vec4;
use crate::patterns::{BoxPattern, Pattern};

use std::any::Any;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub struct StripePattern {
    colors: Vec<Color>,
    transform: Mat4,
    inverse_transform: Mat4,
}

impl StripePattern {
    pub fn new(colors: Vec<Color>, transform: Option<Mat4>) -> Self {
        Self {
            colors,
            transform: transform.unwrap_or_default(),
            inverse_transform: transform
                .unwrap_or_default()
                .inverse()
                .expect("Pattern transform"),
        }
    }

    pub fn new_boxed(colors: Vec<Color>, transform: Option<Mat4>) -> BoxPattern {
        Box::new(Self::new(colors, transform))
    }

    pub fn default_boxed() -> BoxPattern {
        Box::new(Self::default())
    }
}

impl Pattern for StripePattern {
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
        let i = local_point.x.floor().abs() as usize % self.colors.len();
        self.colors[i]
    }
}

impl Default for StripePattern {
    fn default() -> Self {
        Self {
            colors: vec![Color::WHITE, Color::BLACK],
            transform: Mat4::default(),
            inverse_transform: Mat4::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::Sphere;

    #[test]
    fn basic() {
        let pattern = StripePattern::default();
        assert_eq!(pattern.colors[0], Color::WHITE);
        assert_eq!(pattern.colors[1], Color::BLACK);
    }

    #[test]
    fn stripes_y_axis() {
        let pattern = StripePattern::default();
        assert_eq!(pattern.local_pattern_at(Vec4::POINT_ZERO), Color::WHITE);
        assert_eq!(
            pattern.local_pattern_at(Vec4::point(0.0, 1.0, 0.0)),
            Color::WHITE
        );
        assert_eq!(
            pattern.local_pattern_at(Vec4::point(0.0, 2.0, 0.0)),
            Color::WHITE
        );
    }

    #[test]
    fn stripes_z_axis() {
        let pattern = StripePattern::default();
        assert_eq!(pattern.local_pattern_at(Vec4::POINT_ZERO), Color::WHITE);
        assert_eq!(
            pattern.local_pattern_at(Vec4::point(0.0, 0.0, 1.0)),
            Color::WHITE
        );
        assert_eq!(
            pattern.local_pattern_at(Vec4::point(0.0, 0.0, 2.0)),
            Color::WHITE
        );
    }

    #[test]
    fn stripes_alter() {
        let pattern = StripePattern::default();
        assert_eq!(pattern.local_pattern_at(Vec4::POINT_ZERO), Color::WHITE);
        assert_eq!(
            pattern.local_pattern_at(Vec4::point(0.9, 1.0, 0.0)),
            Color::WHITE
        );
        assert_eq!(
            pattern.local_pattern_at(Vec4::point(1.0, 0.0, 0.0)),
            Color::BLACK
        );
        assert_eq!(
            pattern.local_pattern_at(Vec4::point(-0.1, 0.0, 0.0)),
            Color::BLACK
        );
        assert_eq!(
            pattern.local_pattern_at(Vec4::point(-1.0, 0.0, 0.0)),
            Color::BLACK
        );
        assert_eq!(
            pattern.local_pattern_at(Vec4::point(-1.1, 0.0, 0.0)),
            Color::WHITE
        );
    }

    #[test]
    fn three_stripes() {
        let pattern = StripePattern::new(vec![Color::RED, Color::BLUE, Color::GREEN], None);
        assert_eq!(
            pattern.local_pattern_at(Vec4::point(0.0, 1.0, 0.0)),
            Color::RED
        );
        assert_eq!(
            pattern.local_pattern_at(Vec4::point(1.3, 1.0, 0.0)),
            Color::BLUE
        );
        assert_eq!(
            pattern.local_pattern_at(Vec4::point(2.9, 1.0, 0.0)),
            Color::GREEN
        );
    }

    #[test]
    fn on_scaled() {
        let object = Sphere::new(Some(Mat4::scaling(2.0, 2.0, 2.0)), None);
        let pattern = StripePattern::default();
        let c = pattern.pattern_at(&object, Vec4::point(1.5, 0.0, 0.0));
        assert_eq!(c, Color::WHITE);
    }

    #[test]
    fn transformed_pattern() {
        let object = Sphere::default();
        let mut pattern = StripePattern::default();
        pattern.set_transform(Mat4::scaling(2.0, 2.0, 2.0));
        let c = pattern.pattern_at(&object, Vec4::point(1.5, 0.0, 0.0));
        assert_eq!(c, Color::WHITE);
    }

    #[test]
    fn transformed_on_transformed() {
        let object = Sphere::new(Some(Mat4::scaling(2.0, 2.0, 2.0)), None);
        let mut pattern = StripePattern::default();
        pattern.set_transform(Mat4::translation(0.5, 0.0, 0.0));
        let c = pattern.pattern_at(&object, Vec4::point(2.5, 0.0, 0.0));
        assert_eq!(c, Color::WHITE);
    }
}
