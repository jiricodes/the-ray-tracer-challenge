use crate::intersection::{Intersection, Intersections};
use crate::material::Material;
use crate::math::matrix::Mat4;
use crate::math::vec4::Vec4;
use crate::ray::Ray;
use crate::shapes::{BoxShape, Shape};

use std::any::Any;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub transform: Mat4,
    pub inverse_transform: Mat4,
    pub material: Material,
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform && self.material == other.material
    }
}

impl Shape for Sphere {
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
    fn transformation(&self) -> &Mat4 {
        &self.transform
    }
    fn inverse_transformation(&self) -> &Mat4 {
        &self.inverse_transform
    }

    fn local_normal_at(&self, local_point: Vec4) -> Vec4 {
        (local_point - Vec4::POINT_ZERO).normalize()
    }
    fn local_intersect(&self, local_ray: Ray) -> Intersections {
        let sphere_to_ray = local_ray.origin - Vec4::POINT_ZERO;
        let a = local_ray.direction.dot(&local_ray.direction);
        let b = 2.0 * local_ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        let mut ret: Intersections = Intersections::new();
        if discriminant < 0.0 {
            return ret;
        }
        let i = Intersection::new(&self, (-b - discriminant.sqrt()) / (2.0 * a));
        ret.push(i);
        let i = Intersection::new(&self, (-b + discriminant.sqrt()) / (2.0 * a));
        ret.push(i);
        ret
    }
}

impl Sphere {
    pub fn new(transform: Option<Mat4>, material: Option<Material>) -> Self {
        let transform = transform.unwrap_or_default();
        let inverse_transform = transform.inverse().unwrap();
        Self {
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

impl Default for Sphere {
    fn default() -> Self {
        Self {
            transform: Mat4::default(),
            inverse_transform: Mat4::default().inverse().unwrap(),
            material: Material::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn basic() {
        let a = Sphere::default();
        let b = Sphere::default();
        assert_ne!(&a, &b);
        assert_eq!(a.transform, Mat4::IDENTITY);
        assert_eq!(a.material, Material::default());
        assert_eq!(a.material.ambient, 0.1);
    }

    #[test]
    fn ray_intersect() {
        let mut s = Sphere::default();

        // 2 points of intersection
        let mut r = Ray::new(
            &Vec4::new_point(0.0, 0.0, -5.0),
            &Vec4::new_vec(0.0, 0.0, 1.0),
        );
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs.intersections[0].t, 4.0);
        assert_eq!(xs.intersections[1].t, 6.0);

        // tangent
        r.origin = Vec4::new_point(0.0, 1.0, -5.0);
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs.intersections[0].t, 5.0);
        assert_eq!(xs.intersections[1].t, 5.0);

        // no intersection
        r.origin = Vec4::new_point(0.0, 2.0, -5.0);
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 0);

        // ray origin inside sphere
        r.origin = Vec4::new_point(0.0, 0.0, 0.0);
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs.intersections[0].t, -1.0);
        assert_eq!(xs.intersections[1].t, 1.0);

        // ray origin "behind" the sphere
        r.origin = Vec4::new_point(0.0, 0.0, 5.0);
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs.intersections[0].t, -6.0);
        assert_eq!(xs.intersections[1].t, -4.0);

        // After transformation
        s.transform = Mat4::scaling(2.0, 2.0, 2.0);
        r.origin = Vec4::new_point(0.0, 0.0, -5.0);
        r.direction = Vec4::new_vec(0.0, 0.0, 1.0);
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs.intersections[0].t, 3.0);
        assert_eq!(xs.intersections[1].t, 7.0);

        s.transform = Mat4::translation(5.0, 0.0, 0.0);
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn transform() {
        let mut s = Sphere::default();
        let t = Mat4::translation(2.0, 3.0, 4.0);
        s.transform(&t);
        assert_eq!(t, s.transform);
        s.transform = t;
        assert_eq!(t, s.transform);
    }

    #[test]
    fn normal_at() {
        let s = Sphere::default();
        let p = Vec4::new_point(1.0, 0.0, 0.0);
        let exp = Vec4::new_vec(1.0, 0.0, 0.0);
        assert_eq!(exp, s.normal_at(p));

        let p = Vec4::new_point(0.0, 1.0, 0.0);
        let exp = Vec4::new_vec(0.0, 1.0, 0.0);
        assert_eq!(exp, s.normal_at(p));

        let p = Vec4::new_point(0.0, 0.0, 1.0);
        let exp = Vec4::new_vec(0.0, 0.0, 1.0);
        assert_eq!(exp, s.normal_at(p));

        let p = Vec4::new_point(3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0);
        let exp = Vec4::new_vec(3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0);
        let n = s.normal_at(p);
        assert_eq!(exp, n);
        assert_eq!(n, n.normalize());

        let mut s = Sphere::default();

        s.transform = Mat4::translation(0.0, 1.0, 0.0);
        let p = Vec4::new_point(0.0, 1.70711, -0.70711);
        let exp = Vec4::new_vec(0.0, 0.70711, -0.70711);
        let n = s.normal_at(p);
        assert_eq!(exp, n);

        s.transform = Mat4::scaling(1.0, 0.5, 1.0) * Mat4::rotation_z(PI / 5.0);
        let p = Vec4::new_point(0.0, 2f32.sqrt() / 2.0, -2f32.sqrt() / 2.0);
        let exp = Vec4::new_vec(0.0, 0.97014, -0.24254);
        let n = s.normal_at(p);
        assert_eq!(exp, n);
    }
}
