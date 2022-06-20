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
pub struct Cone {
    uid: usize,
    pub transform: Mat4,
    pub inverse_transform: Mat4,
    pub material: Material,
    pub limit_y: (f64, f64),
    pub closed: bool,
}

impl PartialEq for Cone {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
            && self.transform == other.transform
            && self.material == other.material
    }
}

impl Shape for Cone {
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
        let dist = local_point.x.powi(2) + local_point.z.powi(2);
        if dist < 1.0 && local_point.y >= self.limit_y.1 - EPSILON {
            Vec4::VEC_Y_ONE
        } else if dist < 1.0 && local_point.y <= self.limit_y.0 + EPSILON {
            -Vec4::VEC_Y_ONE
        } else {
            Vec4::vec(local_point.x, 0.0, local_point.z)
        }
    }
    fn local_intersect(&self, local_ray: Ray) -> Intersections {
        let mut ret = Intersections::new();

        // a, b, c cone components
        let a = local_ray.direction.x.powi(2) - local_ray.direction.y.powi(2)
            + local_ray.direction.z.powi(2);
        let b = 2.0 * local_ray.origin.x * local_ray.direction.x
            - 2.0 * local_ray.origin.y * local_ray.direction.y
            + 2.0 * local_ray.origin.z * local_ray.direction.z;
        let c =
            local_ray.origin.x.powi(2) - local_ray.origin.y.powi(2) + local_ray.origin.z.powi(2);

        // If a == 0.0 (approx) then the ray is parallel to one of the cone's halves
        if a.abs() < EPSILON {
            // ray misses
            if b.abs() < EPSILON {
                return ret;
            }

            // One point intersection
            let t = -c / (2.0 * b);
            ret.push(Intersection::new(self.box_clone(), t));
            self.intersect_caps(local_ray, &mut ret);
            return ret;
        }

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return ret;
        }

        let t0 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);

        let (t0, t1) = if t0 > t1 { (t1, t0) } else { (t0, t1) };

        let y = local_ray.origin.y + t0 * local_ray.direction.y;
        if y > self.limit_y.0 && y < self.limit_y.1 {
            ret.push(Intersection::new(self.box_clone(), t0));
        }

        let y = local_ray.origin.y + t1 * local_ray.direction.y;
        if y > self.limit_y.0 && y < self.limit_y.1 {
            ret.push(Intersection::new(self.box_clone(), t1));
        }

        // note: we could check if ret.len() < 2 and only then call intersect_caps
        self.intersect_caps(local_ray, &mut ret);
        ret
    }
}

impl Cone {
    pub fn new(
        transform: Option<Mat4>,
        material: Option<Material>,
        limit_y: Option<(f64, f64)>,
        closed: bool,
    ) -> Self {
        let transform = transform.unwrap_or_default();
        let inverse_transform = transform.inverse().unwrap();
        let limit_y = limit_y.unwrap_or_else(|| (-f64::INFINITY, f64::INFINITY));
        Self {
            uid: uid::fetch_uid(),
            transform,
            material: material.unwrap_or_default(),
            inverse_transform,
            limit_y,
            closed,
        }
    }

    pub fn new_boxed(
        transform: Option<Mat4>,
        material: Option<Material>,
        limit_y: Option<(f64, f64)>,
        closed: bool,
    ) -> BoxShape {
        Box::new(Self::new(transform, material, limit_y, closed))
    }

    pub fn default_boxed() -> BoxShape {
        Box::new(Self::default())
    }

    fn intersect_caps(&self, ray: Ray, xs: &mut Intersections) {
        if !self.closed {
            return;
        }

        // Lower cap
        let t = (self.limit_y.0 - ray.origin.y) / ray.direction.y;
        if ray.util_intersection_t_within_radius(t, self.limit_y.0.abs()) {
            xs.push(Intersection::new(self.box_clone(), t))
        }

        // Upper cap
        let t = (self.limit_y.1 - ray.origin.y) / ray.direction.y;
        if ray.util_intersection_t_within_radius(t, self.limit_y.1.abs()) {
            xs.push(Intersection::new(self.box_clone(), t))
        }
    }
}

impl Default for Cone {
    fn default() -> Self {
        Self {
            uid: uid::fetch_uid(),
            transform: Mat4::default(),
            inverse_transform: Mat4::default(),
            material: Material::default(),
            limit_y: (-f64::INFINITY, f64::INFINITY),
            closed: false,
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
        let c1 = Cone::default();
        let c2 = Cone::default();
        assert_ne!(&c1, &c2);
        assert_eq!(c1.transform, Mat4::IDENTITY);
        assert_eq!(c1.material, Material::default());
        assert_eq!(c1.material.ambient, 0.1);
    }

    #[test]
    fn basic_intersect() {
        let cone = Cone::default();

        let dirs = [Vec4::VEC_Z_ONE, Vec4::VEC_ONE, Vec4::vec(-0.5, -1.0, 1.0)];
        let orgs = [
            Vec4::point(0.0, 0.0, -5.0),
            Vec4::point(0.0, 0.0, -5.0),
            Vec4::point(1.0, 1.0, -5.0),
        ];
        let exps = [
            (5.0, 5.0),
            (8.660254037844386, 8.660254037844386),
            (4.550055679356349, 49.449944320643645),
        ];

        for (o, d, et) in izip!(&orgs, &dirs, &exps) {
            let r = Ray::new(o, &d.normalize());
            let xs = cone.local_intersect(r);
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, et.0);
            assert_eq!(xs[1].t, et.1);
        }
    }

    #[test]
    fn basic_parallel_intersect() {
        // Scenario where the ray is parallel to the side of one half
        let cone = Cone::default();
        let r = Ray::new(
            &Vec4::point(0.0, 0.0, -1.0),
            &Vec4::vec(0.0, 1.0, 1.0).normalize(),
        );
        let xs = cone.local_intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 0.3535533905932738);
    }

    #[test]
    fn capped_intersection() {
        let mut cone = Cone::default();
        cone.limit_y = (-0.5, 0.5);
        cone.closed = true;

        let dirs = [Vec4::VEC_Y_ONE, Vec4::vec(0.0, 1.0, 1.0), Vec4::VEC_Y_ONE];
        let orgs = [
            Vec4::point(0.0, 0.0, -5.0),
            Vec4::point(0.0, 0.0, -0.25),
            Vec4::point(0.0, 0.0, -0.25),
        ];
        let exps = [0, 2, 4];

        for (o, d, elen) in izip!(&orgs, &dirs, &exps) {
            let r = Ray::new(o, &d.normalize());
            let xs = cone.local_intersect(r);
            assert_eq!(xs.len(), *elen);
        }
    }
}
