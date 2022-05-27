// use crate::epsilon::EPSILON;
use crate::sphere::Sphere;

// use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection<'a> {
    pub object: &'a Sphere,
    pub t: f32,
}

impl<'a> Intersection<'a> {
    pub fn new(object: &'a Sphere, t: f32) -> Self {
        Self { object, t }
    }
}

// impl PartialEq for Intersection<'_> {
//     fn eq(&self, other: &Intersection) -> bool {
//         (self.t - other.t).abs() < EPSILON
//     }
// }

// impl Eq for Intersection<'_> {}

// impl PartialOrd for Intersection<'_> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         self.t.partial_cmp(&other.t)
//     }
// }

// impl Ord for Intersection<'_> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.partial_cmp(other)
//     }
// }

#[derive(Debug, Clone)]
pub struct Intersections<'a> {
    pub intersections: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    pub fn new() -> Self {
        Self {
            intersections: Vec::<Intersection<'a>>::with_capacity(32),
        }
    }
    pub fn push(&mut self, i: Intersection<'a>) {
        self.intersections.push(i);
    }

    pub fn len(&self) -> usize {
        self.intersections.len()
    }

    pub fn sort(&mut self) {
        self.intersections.sort_by(|a, b| {
            a.t.partial_cmp(&b.t)
                .expect("Partial cmp fail in intersection sort")
        });
    }

    pub fn hit(&mut self) -> Option<&Intersection> {
        self.intersections.iter().find(|i| i.t >= 0.0)
    }

    pub fn clear(&mut self) {
        self.intersections.clear()
    }

    pub fn append(&mut self, other: &mut Self) {
        self.intersections.append(&mut other.intersections);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_inter() {
        let s = Sphere::new();
        let i = Intersection::new(&s, 3.5);
        assert_eq!(i.object, &s);
        assert_eq!(i.t, 3.5);
    }

    #[test]
    fn basic_inters() {
        let s0 = Sphere::new();
        let s1 = Sphere::new();
        let mut inters = Intersections::new();
        let i1 = Intersection::new(&s0, 1.0);
        let i2 = Intersection::new(&s0, 2.0);
        let i3 = Intersection::new(&s1, 1.0);
        let i4 = Intersection::new(&s1, 2.0);
        inters.push(i1);
        inters.push(i2);
        inters.push(i3);
        inters.push(i4);
        assert_eq!(inters.intersections[0].object, &s0);
        assert_eq!(inters.intersections[1].object, &s0);
        assert_eq!(inters.intersections[2].object, &s1);
        assert_eq!(inters.intersections[3].object, &s1);
    }

    #[test]
    fn hits() {
        let s = Sphere::new();
        let mut inters = Intersections::new();

        let i1 = Intersection::new(&s, 1.0);
        let i2 = Intersection::new(&s, 2.0);
        inters.push(i1);
        inters.push(i2);
        inters.sort();
        let i = inters.hit();
        assert_eq!(Some(&i1), i);
        inters.clear();

        let i1 = Intersection::new(&s, -1.0);
        let i2 = Intersection::new(&s, 1.0);
        inters.push(i1);
        inters.push(i2);
        inters.sort();
        let i = inters.hit();
        assert_eq!(Some(&i2), i);
        inters.clear();

        let i1 = Intersection::new(&s, -2.0);
        let i2 = Intersection::new(&s, -1.0);
        inters.push(i1);
        inters.push(i2);
        inters.sort();
        let i = inters.hit();
        assert_eq!(None, i);
        inters.clear();

        let i1 = Intersection::new(&s, 5.0);
        let i2 = Intersection::new(&s, 7.0);
        let i3 = Intersection::new(&s, -3.0);
        let i4 = Intersection::new(&s, 2.0);
        inters.push(i1);
        inters.push(i2);
        inters.push(i3);
        inters.push(i4);
        inters.sort();
        let i = inters.hit();
        assert_eq!(Some(&i4), i);
        inters.clear();
    }
}
