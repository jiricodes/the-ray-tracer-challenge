use crate::ray::Ray;
use crate::vec4::Vec4;
use std::sync::atomic::{AtomicUsize, Ordering};

static SPHERE_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub struct Sphere {
    uid: usize,
    // pub center: Vec4,
    // pub radius: f32,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            uid: SPHERE_COUNTER.fetch_add(1, Ordering::SeqCst),
            // center: Vec4::POINT_ZERO,
            // radius: 1.0,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<f32> {
        let sphere_to_ray = ray.origin - Vec4::POINT_ZERO;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        let mut ret: Vec<f32> = Vec::new();
        if discriminant < 0.0 {
            return ret;
        }
        ret.push((-b - discriminant.sqrt()) / (2.0 * a));
        ret.push((-b + discriminant.sqrt()) / (2.0 * a));
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
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);

        // tangent
        r.origin = Vec4::new_point(0.0, 1.0, -5.0);
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);

        // no intersection
        r.origin = Vec4::new_point(0.0, 2.0, -5.0);
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 0);

        // ray origin inside sphere
        r.origin = Vec4::new_point(0.0, 0.0, 0.0);
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);

        // ray origin "behind" the sphere
        r.origin = Vec4::new_point(0.0, 0.0, 5.0);
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0);
    }
}
