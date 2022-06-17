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

#[derive(Debug, Clone)]
pub struct Cylinder {
    uid: usize,
    pub transform: Mat4,
    pub inverse_transform: Mat4,
    pub material: Material,
}

impl PartialEq for Cylinder {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
            && self.transform == other.transform
            && self.material == other.material
    }
}

impl Shape for Cylinder {
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

    fn local_normal_at(&self, local_point: Vec4) -> Vec4 {
        Vec4::vec(local_point.x, 0.0, local_point.z)
    }
    fn local_intersect(&self, local_ray: Ray) -> Intersections {
        let mut ret = Intersections::new();
        let a = local_ray.direction.x.powi(2) + local_ray.direction.z.powi(2);
        if a.abs() < EPSILON {
            return ret;
        }
        let b = 2.0 * local_ray.origin.x * local_ray.direction.x
            + 2.0 * local_ray.origin.z * local_ray.direction.z;
        let c = local_ray.origin.x.powi(2) + local_ray.origin.z.powi(2) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return ret;
        }
        ret.push(Intersection::new(
            self.box_clone(),
            (-b - discriminant.sqrt()) / (2.0 * a),
        ));
        ret.push(Intersection::new(
            self.box_clone(),
            (-b + discriminant.sqrt()) / (2.0 * a),
        ));
        ret
    }
}

impl Cylinder {
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

impl Default for Cylinder {
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
    use itertools::izip;
    use std::f64::consts::{PI, SQRT_2};

    #[test]
    fn basic() {
        let c1 = Cylinder::default();
        let c2 = Cylinder::default();
        assert_ne!(&c1, &c2);
        assert_eq!(c1.transform, Mat4::IDENTITY);
        assert_eq!(c1.material, Material::default());
        assert_eq!(c1.material.ambient, 0.1);
    }

    #[test]
    fn ray_miss() {
        let c = Cylinder::default();
        let dirs = [Vec4::VEC_Y_ONE, Vec4::VEC_Y_ONE, Vec4::VEC_ONE];
        let orgs = [
            Vec4::point(1.0, 0.0, 0.0),
            Vec4::POINT_ZERO,
            Vec4::point(0.0, 0.0, -5.0),
        ];
        for (o, d) in izip!(&orgs, &dirs) {
            let r = Ray::new(o, &d.normalize());
            let xs = c.local_intersect(r);
            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn ray_intersect() {
        let c = Cylinder::default();
        let dirs = [Vec4::VEC_Z_ONE, Vec4::VEC_Z_ONE, Vec4::vec(0.1, 1.0, 1.0)];
        let orgs = [
            Vec4::point(1.0, 0.0, -5.0),
            Vec4::point(0.0, 0.0, -5.0),
            Vec4::point(0.5, 0.0, -5.0),
        ];
        let exps = [
            (5.0, 5.0),
            (4.0, 6.0),
            (6.80798191702732, 7.088723439378861),
        ];
        for (o, d, et) in izip!(&orgs, &dirs, &exps) {
            let r = Ray::new(o, &d.normalize());
            let xs = c.local_intersect(r);
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, et.0);
            assert_eq!(xs[1].t, et.1);
        }
    }

    #[test]
    fn normal_at() {
        let c = Cylinder::default();
        let points = [
            Vec4::point(1.0, 0.0, 0.0),
            Vec4::point(0.0, 5.0, -1.0),
            Vec4::point(0.0, -2.0, 1.0),
            Vec4::point(-1.0, 1.0, 0.0),
        ];
        let exps = [
            Vec4::VEC_X_ONE,
            -Vec4::VEC_Z_ONE,
            Vec4::VEC_Z_ONE,
            -Vec4::VEC_X_ONE,
        ];
        for (p, exp) in izip!(&points, &exps) {
            let n = c.local_normal_at(*p);
            assert_eq!(n, *exp);
        }
    }
}
