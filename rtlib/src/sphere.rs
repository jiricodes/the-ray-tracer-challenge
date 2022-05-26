use crate::intersection::{Intersection, Intersections};
use crate::matrix::Mat4;
use crate::ray::Ray;
use crate::vec4::Vec4;
use std::sync::atomic::{AtomicUsize, Ordering};

static SPHERE_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, PartialEq)]
pub struct Sphere {
    uid: usize,
    transform: Mat4,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            uid: SPHERE_COUNTER.fetch_add(1, Ordering::SeqCst),
            transform: Mat4::IDENTITY,
        }
    }

    pub fn intersect<'a>(&'a self, ray: &Ray) -> Intersections {
        let sphere_to_ray = ray.origin - Vec4::POINT_ZERO;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let a = Sphere::new();
        let b = Sphere::new();
        assert_ne!(a.uid, b.uid);
        assert_eq!(a.transform, Mat4::IDENTITY);
    }

    #[test]
    fn ray_intersect() {
        let s = Sphere::new();

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
    }

    #[test]
    fn transform() {}
}
