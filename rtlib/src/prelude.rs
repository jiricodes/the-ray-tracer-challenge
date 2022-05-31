// structs
pub use crate::camera::Camera;
pub use crate::canvas::Canvas;
pub use crate::color::Color;
pub use crate::intersection::Intersection;
pub use crate::light::PointLight;
pub use crate::material::Material;
pub use crate::math::matrix::Mat4;
pub use crate::math::vec4::Vec4;
pub use crate::ray::Ray;
pub use crate::shapes::sphere::Sphere;
pub use crate::world::World;

// Traits
pub use crate::shapes::Shape;

// functions
pub use crate::render::render;

// Consts
pub use crate::math::EPSILON;
pub use std::f64::consts::PI;
