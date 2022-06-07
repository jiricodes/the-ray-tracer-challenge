use crate::color::Color;
use crate::math::matrix::Mat4;
use crate::math::vec4::Vec4;
use crate::shapes::Shape;

use std::any::Any;
use std::fmt::Debug;

pub mod stripes;
pub use stripes::StripePattern;

pub mod gradient;
pub use gradient::GradientPattern;

pub mod ring;
pub use ring::RingPattern;

pub mod checkers;
pub use checkers::CheckersPattern;

pub trait Pattern: Any + Debug {
    fn as_any(&self) -> &dyn Any;
    fn box_clone(&self) -> BoxPattern;
    fn box_eq(&self, other: &dyn Any) -> bool;

    fn transform(&mut self, m: &Mat4);
    fn set_transform(&mut self, transformation: Mat4);
    fn transformation(&self) -> &Mat4;
    fn inverse_transformation(&self) -> &Mat4;

    fn local_pattern_at(&self, local_point: Vec4) -> Color;

    fn pattern_at(&self, object: &dyn Shape, world_point: Vec4) -> Color {
        let object_point = object.inverse_transformation() * world_point;
        let local_point = self.inverse_transformation() * object_point;
        self.local_pattern_at(local_point)
    }
}

pub type BoxPattern = Box<dyn Pattern>;

impl Clone for BoxPattern {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl PartialEq for BoxPattern {
    fn eq(&self, other: &BoxPattern) -> bool {
        self.box_eq(other.as_any())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::material::Material;
    use crate::shapes;

    #[derive(Debug, PartialEq, Clone)]
    pub struct TestPattern {
        color: Color,
        transform: Mat4,
        inverse_transform: Mat4,
    }

    impl TestPattern {
        pub fn new(color: Color, transform: Option<Mat4>) -> Self {
            Self {
                color,
                transform: transform.unwrap_or_default(),
                inverse_transform: transform
                    .unwrap_or_default()
                    .inverse()
                    .expect("Pattern transform"),
            }
        }

        pub fn new_boxed(color: Color, transform: Option<Mat4>) -> BoxPattern {
            Box::new(Self::new(color, transform))
        }

        pub fn default_boxed() -> BoxPattern {
            Box::new(Self::default())
        }
    }

    impl Pattern for TestPattern {
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
            Color::rgb(local_point.x, local_point.y, local_point.z)
        }
    }

    impl Default for TestPattern {
        fn default() -> Self {
            Self {
                color: Color::WHITE,
                transform: Mat4::default(),
                inverse_transform: Mat4::default(),
            }
        }
    }

    #[test]
    fn testpattern_basic() {
        let p = TestPattern::default();
        assert_eq!(p.transform, Mat4::IDENTITY);
    }

    #[test]
    fn testpattern_transform() {
        let mut p = TestPattern::new(Color::WHITE, Some(Mat4::translation(1.0, 2.0, 3.0)));
        assert_eq!(p.transform, Mat4::translation(1.0, 2.0, 3.0));
        let m = Mat4::scaling(5.0, 4.2, 3.3);
        p.set_transform(m.clone());
        assert_eq!(p.transform, m);
    }

    #[test]
    fn testpattern_color_at_scaled() {
        let p = TestPattern::default_boxed();
        let s = shapes::Sphere::new(
            Some(Mat4::scaling(2.0, 2.0, 2.0)),
            Some(Material {
                pattern: Some(p.clone()),
                ..Default::default()
            }),
        );
        assert_eq!(
            p.pattern_at(&s, Vec4::point(2.0, 3.0, 4.0)),
            Color::rgb(1.0, 1.5, 2.0)
        );
    }

    #[test]
    fn testpattern_scaled_color_at() {
        let p = TestPattern::new_boxed(Color::WHITE, Some(Mat4::scaling(2.0, 2.0, 2.0)));
        let s = shapes::Sphere::new(
            None,
            Some(Material {
                pattern: Some(p.clone()),
                ..Default::default()
            }),
        );
        assert_eq!(
            p.pattern_at(&s, Vec4::point(2.0, 3.0, 4.0)),
            Color::rgb(1.0, 1.5, 2.0)
        );
    }

    #[test]
    fn testpattern_scaled_color_at_scaled() {
        let p = TestPattern::new_boxed(Color::WHITE, Some(Mat4::translation(0.5, 1.0, 1.5)));
        let s = shapes::Sphere::new(
            Some(Mat4::scaling(2.0, 2.0, 2.0)),
            Some(Material {
                pattern: Some(p.clone()),
                ..Default::default()
            }),
        );
        assert_eq!(
            p.pattern_at(&s, Vec4::point(2.5, 3.0, 3.5)),
            Color::rgb(0.75, 0.5, 0.25)
        );
    }
}
