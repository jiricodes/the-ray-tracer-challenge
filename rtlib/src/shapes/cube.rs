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
pub struct Cube {
    uid: usize,
    pub transform: Mat4,
    pub inverse_transform: Mat4,
    pub material: Material,
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
            && self.transform == other.transform
            && self.material == other.material
    }
}

impl Shape for Cube {
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
        let abs_vec = local_point.abs();
        let max_c = abs_vec.x.max(abs_vec.y.max(abs_vec.z));
        if max_c == local_point.x.abs() {
            Vec4::vec(local_point.x, 0.0, 0.0)
        } else if max_c == local_point.y.abs() {
            Vec4::vec(0.0, local_point.y, 0.0)
        } else {
            Vec4::vec(0.0, 0.0, local_point.z)
        }
    }

    fn local_intersect(&self, local_ray: Ray) -> Intersections {
        let (mut tmin, mut tmax) = Self::check_axis(local_ray.origin.x, local_ray.direction.x);
        let (cmin, cmax) = Self::check_axis(local_ray.origin.y, local_ray.direction.y);
        tmin = tmin.max(cmin);
        tmax = tmax.min(cmax);
        let (cmin, cmax) = Self::check_axis(local_ray.origin.z, local_ray.direction.z);
        tmin = tmin.max(cmin);
        tmax = tmax.min(cmax);

        if tmin > tmax {
            return Intersections::new();
        } else {
            Intersections::from(vec![
                Intersection::new(self.box_clone(), tmin),
                Intersection::new(self.box_clone(), tmax),
            ])
        }
    }
}

impl Cube {
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

    fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
        let tmin_num = -1.0 - origin;
        let tmax_num = 1.0 - origin;

        let (tmin, tmax) = if direction.abs() >= EPSILON {
            (tmin_num / direction, tmax_num / direction)
        } else {
            (tmin_num * std::f64::INFINITY, tmax_num * std::f64::INFINITY)
        };
        if tmin > tmax {
            (tmax, tmin)
        } else {
            (tmin, tmax)
        }
    }
}

impl Default for Cube {
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
    use std::f64::consts::{PI, SQRT_2};

    use itertools::izip;

    #[test]
    fn basic() {
        let a = Cube::default();
        let b = Cube::default();
        assert_ne!(&a, &b);
        assert_eq!(a.transform, Mat4::IDENTITY);
        assert_eq!(a.material, Material::default());
        assert_eq!(a.material.ambient, 0.1);
    }

    #[test]
    fn ray_intersect() {
        let c = Cube::default();

        let ray_origins = vec![
            Vec4::point(5.0, 0.5, 0.0),
            Vec4::point(-5.0, 0.5, 0.0),
            Vec4::point(0.5, 5.0, 0.0),
            Vec4::point(0.5, -5.0, 0.0),
            Vec4::point(0.5, 0.0, 5.0),
            Vec4::point(0.5, 0.0, -5.0),
            Vec4::point(0.0, 0.5, 0.0),
        ];

        let ray_directions = vec![
            -Vec4::VEC_X_ONE,
            Vec4::VEC_X_ONE,
            -Vec4::VEC_Y_ONE,
            Vec4::VEC_Y_ONE,
            -Vec4::VEC_Z_ONE,
            Vec4::VEC_Z_ONE,
            Vec4::VEC_Z_ONE,
        ];

        let exp_t1 = vec![4.0, 4.0, 4.0, 4.0, 4.0, 4.0, -1.0];
        let exp_t2 = vec![6.0, 6.0, 6.0, 6.0, 6.0, 6.0, 1.0];

        for (org, dir, t1, t2) in izip!(&ray_origins, &ray_directions, &exp_t1, &exp_t2) {
            let r = Ray::new(org, dir);
            let xs = c.intersect(&r);
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, *t1);
            assert_eq!(xs[1].t, *t2);
        }
    }

    #[test]
    fn ray_miss() {
        let c = Cube::default();

        let ray_origins = vec![
            Vec4::point(-2.0, 0.0, 0.0),
            Vec4::point(0.0, -2.0, 0.0),
            Vec4::point(0.0, 0.0, -2.0),
            Vec4::point(2.0, 0.0, 2.0),
            Vec4::point(0.0, 2.0, 2.0),
            Vec4::point(2.0, 2.0, 0.0),
        ];

        let ray_directions = vec![
            Vec4::vec(0.2673, 0.5345, 0.8018),
            Vec4::vec(0.8018, 0.2673, 0.5345),
            Vec4::vec(0.5345, 0.8018, 0.2673),
            -Vec4::VEC_Z_ONE,
            -Vec4::VEC_Y_ONE,
            -Vec4::VEC_X_ONE,
        ];

        for (org, dir) in izip!(&ray_origins, &ray_directions) {
            let r = Ray::new(org, dir);
            let xs = c.intersect(&r);
            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn normal_at_surface() {
        let c = Cube::default();

        let points = vec![
            Vec4::point(1.0, 0.5, -0.8),
            Vec4::point(-1.0, -0.2, 0.9),
            Vec4::point(-0.4, 1.0, -0.1),
            Vec4::point(0.3, -1.0, -0.7),
            Vec4::point(-0.6, 0.3, 1.0),
            Vec4::point(0.4, 0.4, -1.0),
            Vec4::point(1.0, 1.0, 1.0),
            Vec4::point(-1.0, -1.0, -1.0),
        ];

        let exp = vec![
            Vec4::VEC_X_ONE,
            -Vec4::VEC_X_ONE,
            Vec4::VEC_Y_ONE,
            -Vec4::VEC_Y_ONE,
            Vec4::VEC_Z_ONE,
            -Vec4::VEC_Z_ONE,
            Vec4::VEC_X_ONE,
            -Vec4::VEC_X_ONE,
        ];

        for (p, e) in izip!(&points, &exp) {
            assert_eq!(c.local_normal_at(*p), *e, "Failed for {:?}", p);
        }
    }

    #[test]
    fn normal_at_scaled_rotated() {}
}
