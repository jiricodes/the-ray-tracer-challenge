//! Module to handle intersections and their related computations
//!

use crate::color::Color;
use crate::light::PointLight;
use crate::math::vec4::Vec4;
use crate::math::EPSILON;
use crate::ray::Ray;
use crate::shapes::BoxShape;

use std::ops::Index;

#[derive(Debug, Clone)]
pub struct Intersection {
    pub object: BoxShape,
    pub t: f64,
}

impl Intersection {
    pub fn new(object: BoxShape, t: f64) -> Self {
        Self { object, t }
    }

    pub fn precomputed(&self, ray: Ray) -> IntersectionComps {
        unimplemented!()
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        (self.t - other.t).abs() < EPSILON && &self.object == &other.object
    }
}

#[derive(Debug, Clone)]
pub struct Intersections {
    inner: Vec<Intersection>,
}

impl Index<usize> for Intersections {
    type Output = Intersection;
    fn index(&self, i: usize) -> &Self::Output {
        &self.inner[i]
    }
}

impl Intersections {
    pub fn new() -> Self {
        Self {
            inner: Vec::<Intersection>::with_capacity(32),
        }
    }
    pub fn push(&mut self, i: Intersection) {
        self.inner.push(i);
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn sort(&mut self) {
        self.inner.sort_by(|a, b| {
            a.t.partial_cmp(&b.t)
                .expect("Partial cmp fail in intersection sort")
        });
    }

    pub fn hit(&self) -> Option<&Intersection> {
        self.inner.iter().find(|i| i.t >= 0.0)
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }

    pub fn append(&mut self, other: &mut Self) {
        self.inner.append(&mut other.inner);
    }
}

/// Collection of precomputed values of an intersection.
#[derive(Debug)]
pub struct IntersectionComps {
    t: f64,
    object: BoxShape,
    point: Vec4,
    eye_vec: Vec4,
    normal: Vec4,
    inside: bool,
    over_point: Vec4,
}

impl IntersectionComps {
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
            t: i.t,
            object: i.object,
            point: p,
            eye_vec: e,
            normal: n,
            inside,
            over_point: p + (n * EPSILON),
        }
    }

    pub fn lighting(&self, light: &PointLight, in_shadow: bool) -> Color {
        self.object.get_material().lighting(
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
    use crate::shapes::Sphere;

    #[test]
    fn basic_inter() {
        let s = Sphere::default_boxed();
        let i = Intersection::new(s, 3.5);
        assert_eq!(i.object, s);
        assert_eq!(i.t, 3.5);
    }

    #[test]
    fn basic_inters() {
        let s0 = Sphere::default_boxed();
        let s1 = Sphere::default_boxed();
        let mut inters = Intersections::new();
        let i1 = Intersection::new(s0, 1.0);
        let i2 = Intersection::new(s0, 2.0);
        let i3 = Intersection::new(s1, 1.0);
        let i4 = Intersection::new(s1, 2.0);
        inters.push(i1);
        inters.push(i2);
        inters.push(i3);
        inters.push(i4);
        assert_eq!(inters[0].object, s0);
        assert_eq!(inters[1].object, s0);
        assert_eq!(inters[2].object, s1);
        assert_eq!(inters[3].object, s1);
    }

    #[test]
    fn hits() {
        let s = Sphere::default_boxed();
        let mut inters = Intersections::new();

        let i1 = Intersection::new(s, 1.0);
        let i2 = Intersection::new(s, 2.0);
        inters.push(i1);
        inters.push(i2);
        inters.sort();
        let i = inters.hit();
        assert_eq!(Some(&i1), i);
        inters.clear();

        let i1 = Intersection::new(s, -1.0);
        let i2 = Intersection::new(s, 1.0);
        inters.push(i1);
        inters.push(i2);
        inters.sort();
        let i = inters.hit();
        assert_eq!(Some(&i2), i);
        inters.clear();

        let i1 = Intersection::new(s, -2.0);
        let i2 = Intersection::new(s, -1.0);
        inters.push(i1);
        inters.push(i2);
        inters.sort();
        let i = inters.hit();
        assert_eq!(None, i);
        inters.clear();

        let i1 = Intersection::new(s, 5.0);
        let i2 = Intersection::new(s, 7.0);
        let i3 = Intersection::new(s, -3.0);
        let i4 = Intersection::new(s, 2.0);
        inters.push(i1);
        inters.push(i2);
        inters.push(i3);
        inters.push(i4);
        inters.sort();
        let i = inters.hit();
        assert_eq!(Some(&i4), i);
        inters.clear();
    }

    #[test]
    fn precomputed() {
        let r = Ray::new(
            &Vec4::new_point(0.0, 0.0, -5.0),
            &Vec4::new_vec(0.0, 0.0, 1.0),
        );

        let s = Sphere::default_boxed();
        let i = Intersection::new(s, 4.0);
        let comps = IntersectionComps::new(&i, &r);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Vec4::new_point(0.0, 0.0, -1.0));
        assert_eq!(comps.eye_vec, Vec4::new_vec(0.0, 0.0, -1.0));
        assert_eq!(comps.normal, Vec4::new_vec(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, false);

        let r = Ray::new(&Vec4::POINT_ZERO, &Vec4::new_vec(0.0, 0.0, 1.0));

        let s = Sphere::default_boxed();
        let i = Intersection::new(s, 1.0);
        let comps = IntersectionComps::new(&i, &r);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Vec4::new_point(0.0, 0.0, 1.0));
        assert_eq!(comps.eye_vec, Vec4::new_vec(0.0, 0.0, -1.0));
        assert_eq!(comps.normal, Vec4::new_vec(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
    }

    #[test]
    fn overpoint() {
        let r = Ray::new(
            &Vec4::new_point(0.0, 0.0, -5.0),
            &Vec4::new_vec(0.0, 0.0, 1.0),
        );
        let mut s = Sphere::default_boxed();
        s.set_transform(Mat4::translation(0.0, 0.0, 1.0));
        let i = Intersection::new(s, 5.0);
        let comps = IntersectionComps::new(&i, &r);
        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}
