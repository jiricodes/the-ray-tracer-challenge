use crate::color::Color;
use crate::intersection::Intersection;
use crate::light::PointLight;
use crate::math::vec4::Vec4;
use crate::math::EPSILON;
use crate::ray::Ray;
use crate::shapes::BoxShape;

#[derive(Debug)]
pub struct PreCompute {
    _t: f64,
    object: BoxShape,
    _point: Vec4,
    eye_vec: Vec4,
    normal: Vec4,
    _inside: bool,
    over_point: Vec4,
}

impl PreCompute {
    pub fn new(i: &Intersection, r: &Ray) -> Self {
        let p = r.position(i.t);
        let mut n = i.object.normal_at(p);
        let e = -r.direction;
        let mut inside = false;
        if n.dot(&e) < 0.0 {
            inside = true;
            n = -n;
        }
        Self {
            _t: i.t,
            object: i.object.clone(),
            _point: p,
            eye_vec: e,
            normal: n,
            _inside: inside,
            over_point: p + (n * EPSILON),
        }
    }

    pub fn lighting(&self, light: &PointLight, in_shadow: bool) -> Color {
        self.object.get_material().lighting(
            &*(self.object),
            &self.over_point,
            light,
            &self.eye_vec,
            &self.normal,
            in_shadow,
        )
    }

    pub fn get_overpoint(&self) -> &Vec4 {
        &self.over_point
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::matrix::Mat4;
    use crate::shapes::{Plane, Sphere};

    #[test]
    fn precomputed() {
        let r = Ray::new(&Vec4::point(0.0, 0.0, -5.0), &Vec4::vec(0.0, 0.0, 1.0));

        let s = Sphere::default_boxed();
        let i = Intersection::new(s, 4.0);
        let comps = i.precomputed(&r);
        assert_eq!(comps._t, i.t);
        assert_eq!(&comps.object, &i.object);
        assert_eq!(comps._point, Vec4::point(0.0, 0.0, -1.0));
        assert_eq!(comps.eye_vec, Vec4::vec(0.0, 0.0, -1.0));
        assert_eq!(comps.normal, Vec4::vec(0.0, 0.0, -1.0));
        assert_eq!(comps._inside, false);

        let r = Ray::new(&Vec4::POINT_ZERO, &Vec4::vec(0.0, 0.0, 1.0));

        let s = Sphere::default_boxed();
        let i = Intersection::new(s, 1.0);
        let comps = i.precomputed(&r);
        assert_eq!(comps._t, i.t);
        assert_eq!(&comps.object, &i.object);
        assert_eq!(comps._point, Vec4::point(0.0, 0.0, 1.0));
        assert_eq!(comps.eye_vec, Vec4::vec(0.0, 0.0, -1.0));
        assert_eq!(comps.normal, Vec4::vec(0.0, 0.0, -1.0));
        assert_eq!(comps._inside, true);
    }

    #[test]
    fn overpoint() {
        let r = Ray::new(&Vec4::point(0.0, 0.0, -5.0), &Vec4::vec(0.0, 0.0, 1.0));
        let mut s = Sphere::default_boxed();
        s.set_transform(Mat4::translation(0.0, 0.0, 1.0));
        let i = Intersection::new(s, 5.0);
        let comps = i.precomputed(&r);
        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps._point.z > comps.over_point.z);
    }

    #[test]
    fn reflection() {
        let plane = Plane::default_boxed();
        // let ray = Ray::new()
    }
}
