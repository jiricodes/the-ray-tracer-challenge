use crate::color::Color;
use crate::math::matrix::Mat4;
use crate::math::vec4::Vec4;
use crate::shapes::Shape;

use std::any::Any;
use std::fmt::Debug;

pub mod stripes;
pub use stripes::StripePattern;

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
