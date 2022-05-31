use crate::intersection::{Intersection, Intersections};
use crate::material::Material;
use crate::matrix::Mat4;
use crate::ray::Ray;
use crate::util::uid;
use crate::vec4::Vec4;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    uid: usize,
    pub transform: Mat4,
    pub material: Material,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            uid: uid::fetch_uid(),
            transform: Mat4::IDENTITY,
            material: Material::default(),
        }
    }

    pub fn with_material(material: Material) -> Self {
        let mut new = Self::new();
        new.material = material;
        new
    }

    pub fn get_uid(&self) -> usize {
        self.uid
    }

    pub fn intersect<'a>(&'a self, ray: &Ray) -> Intersections {
        let ray = ray.transform(
            &self
                .transform
                .inverse()
                .expect("Object transform matrix is not invertible"),
        );
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

    pub fn transform(&mut self, m: &Mat4) {
        self.transform = m * self.transform;
    }

    /// Assumes p on the surface
    pub fn normal_at(&self, p: &Vec4) -> Vec4 {
        let it = self
            .transform
            .inverse()
            .expect("Object transform matrix is not invertible");
        let op = it * *p;
        let on = (op - Vec4::POINT_ZERO).normalize();
        let mut normal = it.transpose() * on;
        normal.w = 0.0;
        normal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn basic() {
        let a = Sphere::new();
        let b = Sphere::new();
        assert_ne!(a.uid, b.uid);
        assert_eq!(a.transform, Mat4::IDENTITY);
        assert_eq!(a.material, Material::default());
        assert_eq!(a.material.ambient, 0.1);
    }

    #[test]
    fn ray_intersect() {
        let mut s = Sphere::new();

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
        let mut s = Sphere::new();
        let t = Mat4::translation(2.0, 3.0, 4.0);
        s.transform(&t);
        assert_eq!(t, s.transform);
        s.transform = t;
        assert_eq!(t, s.transform);
    }

    #[test]
    fn normal_at() {
        let s = Sphere::new();
        let p = Vec4::new_point(1.0, 0.0, 0.0);
        let exp = Vec4::new_vec(1.0, 0.0, 0.0);
        assert_eq!(exp, s.normal_at(&p));

        let p = Vec4::new_point(0.0, 1.0, 0.0);
        let exp = Vec4::new_vec(0.0, 1.0, 0.0);
        assert_eq!(exp, s.normal_at(&p));

        let p = Vec4::new_point(0.0, 0.0, 1.0);
        let exp = Vec4::new_vec(0.0, 0.0, 1.0);
        assert_eq!(exp, s.normal_at(&p));

        let p = Vec4::new_point(3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0);
        let exp = Vec4::new_vec(3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0, 3f32.sqrt() / 3.0);
        let n = s.normal_at(&p);
        assert_eq!(exp, n);
        assert_eq!(n, n.normalize());

        let mut s = Sphere::new();

        s.transform = Mat4::translation(0.0, 1.0, 0.0);
        let p = Vec4::new_point(0.0, 1.70711, -0.70711);
        let exp = Vec4::new_vec(0.0, 0.70711, -0.70711);
        let n = s.normal_at(&p);
        assert_eq!(exp, n);

        s.transform = Mat4::scaling(1.0, 0.5, 1.0) * Mat4::rotation_z(PI / 5.0);
        let p = Vec4::new_point(0.0, 2f32.sqrt() / 2.0, -2f32.sqrt() / 2.0);
        let exp = Vec4::new_vec(0.0, 0.97014, -0.24254);
        let n = s.normal_at(&p);
        assert_eq!(exp, n);
    }
}
