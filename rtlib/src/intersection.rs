//! Module to handle intersections and their related computations
//!

use crate::math::EPSILON;
use crate::precompute::PreCompute;
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

    pub fn precomputed(&self, ray: &Ray) -> PreCompute {
        PreCompute::new(&self, ray)
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

impl From<Vec<Intersection>> for Intersections {
    fn from(xs: Vec<Intersection>) -> Self {
        Self { inner: xs }
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

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::Sphere;

    #[test]
    fn basic_inter() {
        let s = Sphere::default_boxed();
        let i = Intersection::new(s.clone(), 3.5);
        assert_eq!(&i.object, &s);
        assert_eq!(i.t, 3.5);
    }

    #[test]
    fn basic_inters() {
        let s0 = Sphere::default_boxed();
        let s1 = Sphere::default_boxed();
        let mut inters = Intersections::new();
        let i1 = Intersection::new(s0.clone(), 1.0);
        let i2 = Intersection::new(s0.clone(), 2.0);
        let i3 = Intersection::new(s1.clone(), 1.0);
        let i4 = Intersection::new(s1.clone(), 2.0);
        inters.push(i1);
        inters.push(i2);
        inters.push(i3);
        inters.push(i4);
        assert_eq!(&inters[0].object, &s0);
        assert_eq!(&inters[1].object, &s0);
        assert_eq!(&inters[2].object, &s1);
        assert_eq!(&inters[3].object, &s1);
    }

    #[test]
    fn hits() {
        let s = Sphere::default_boxed();
        let mut inters = Intersections::new();

        let i1 = Intersection::new(s.clone(), 1.0);
        let i2 = Intersection::new(s.clone(), 2.0);
        inters.push(i1.clone());
        inters.push(i2);
        inters.sort();
        let i = inters.hit();
        assert_eq!(Some(&i1), i);
        inters.clear();

        let i1 = Intersection::new(s.clone(), -1.0);
        let i2 = Intersection::new(s.clone(), 1.0);
        inters.push(i1);
        inters.push(i2.clone());
        inters.sort();
        let i = inters.hit();
        assert_eq!(Some(&i2), i);
        inters.clear();

        let i1 = Intersection::new(s.clone(), -2.0);
        let i2 = Intersection::new(s.clone(), -1.0);
        inters.push(i1);
        inters.push(i2);
        inters.sort();
        let i = inters.hit();
        assert_eq!(None, i);
        inters.clear();

        let i1 = Intersection::new(s.clone(), 5.0);
        let i2 = Intersection::new(s.clone(), 7.0);
        let i3 = Intersection::new(s.clone(), -3.0);
        let i4 = Intersection::new(s.clone(), 2.0);
        inters.push(i1);
        inters.push(i2);
        inters.push(i3);
        inters.push(i4.clone());
        inters.sort();
        let i = inters.hit();
        assert_eq!(Some(&i4), i);
        inters.clear();
    }
}
