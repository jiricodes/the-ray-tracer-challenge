use crate::intersection::{Intersection, Intersections};
use crate::material::Material;
use crate::math::matrix::Mat4;
use crate::math::vec4::Vec4;
use crate::math::EPSILON;
use crate::ray::Ray;
use crate::shapes::{BoxShape, Shape};
use crate::util::uid;

use std::any::Any;
use std::fmt::Debug;

/// A *plane* is a perfectly flat surface tha extends infinitely in two dimensions.
/// The default plane is considered to be xz-plane.
#[derive(Debug, Clone)]
pub struct Plane {
    uid: usize,
    pub transform: Mat4,
    pub inverse_transform: Mat4,
    pub material: Material,
}

impl PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
            && self.transform == other.transform
            && self.material == other.material
    }
}

impl Shape for Plane {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn box_clone(&self) -> BoxShape {
        Box::new((*self).clone())
    }
    fn box_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }
    fn get_material(&self) -> &Material {
        &self.material
    }

    fn transform(&mut self, m: &Mat4) {
        self.transform = m * self.transform;
    }
    fn set_transform(&mut self, transform: Mat4) {
        self.transform = transform;
        self.inverse_transform = self.transform.inverse().unwrap();
    }
    fn transformation(&self) -> &Mat4 {
        &self.transform
    }
    fn inverse_transformation(&self) -> &Mat4 {
        &self.inverse_transform
    }

    fn local_normal_at(&self, _local_point: Vec4) -> Vec4 {
        Vec4::VEC_Y_ONE
    }
    fn local_intersect(&self, local_ray: Ray) -> Intersections {
        if local_ray.direction.y.abs() < EPSILON {
            return Intersections::new();
        }
        let t = -local_ray.origin.y / local_ray.direction.y;
        vec![Intersection::new(Box::new(self.clone()), t)].into()
    }
}

impl Plane {
    pub fn new(transform: Option<Mat4>, material: Option<Material>) -> Self {
        let transform = transform.unwrap_or_default();
        let inverse_transform = transform.inverse().unwrap();
        Self {
            uid: uid::fetch_uid(),
            transform,
            material: material.unwrap_or_default(),
            inverse_transform,
        }
    }

    pub fn new_boxed(transform: Option<Mat4>, material: Option<Material>) -> BoxShape {
        Box::new(Self::new(transform, material))
    }

    pub fn default_boxed() -> BoxShape {
        Box::new(Self::default())
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            uid: uid::fetch_uid(),
            transform: Mat4::default(),
            inverse_transform: Mat4::default(),
            material: Material::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_is_const() {
        let plane = Plane::default();
        let exp = Vec4::VEC_Y_ONE;
        assert_eq!(plane.local_normal_at(Vec4::point(0.0, 0.0, 0.0)), exp);
        assert_eq!(
            plane.local_normal_at(Vec4::point(10.0, 0.0, -10.0)),
            exp
        );
        assert_eq!(
            plane.local_normal_at(Vec4::point(-5.0, 0.0, 150.0)),
            exp
        );
    }

    #[test]
    fn intersect_parallel() {
        let plane = Plane::default();
        let r = Ray::new(&Vec4::point(0.0, 10.0, 0.0), &Vec4::VEC_Z_ONE);
        let xs = plane.local_intersect(r);
        assert!(xs.is_empty())
    }

    #[test]
    fn intersect_coplanar() {
        let plane = Plane::default();
        let r = Ray::new(&Vec4::POINT_ZERO, &Vec4::VEC_Z_ONE);
        let xs = plane.local_intersect(r);
        assert!(xs.is_empty())
    }

    #[test]
    fn intersect_above() {
        let plane = Plane::default_boxed();
        let r = Ray::new(&Vec4::point(0.0, 1.0, 0.0), &-Vec4::VEC_Y_ONE);
        let xs = plane.local_intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(&xs[0].object, &plane);
    }

    #[test]
    fn intersect_below() {
        let plane = Plane::default_boxed();
        let r = Ray::new(&Vec4::point(0.0, -1.0, 0.0), &Vec4::VEC_Y_ONE);
        let xs = plane.local_intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(&xs[0].object, &plane);
    }
}
