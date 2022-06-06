use crate::color::Color;
use crate::intersection::Intersection;
use crate::light::PointLight;
use crate::material::Material;
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
    under_point: Vec4,
    reflect_vec: Vec4,
    n1: f64,
    n2: f64,
}

impl PreCompute {
    pub fn new(i: &Intersection, r: &Ray, xs: Option<&Vec<Intersection>>) -> Self {
        let p = r.position(i.t);
        let mut normal = i.object.normal_at(p);
        let e = -r.direction;
        let mut inside = false;
        if normal.dot(&e) < 0.0 {
            inside = true;
            normal = -normal;
        }

        // n1 and n2 checking
        let tmp_xs = vec![i.clone()];
        let xs: &Vec<Intersection> = if xs.is_none() { &tmp_xs } else { xs.unwrap() };

        let mut n1: f64 = 0.0;
        let mut n2: f64 = 0.0;
        let mut containers: Vec<BoxShape> = Vec::new();
        for xi in xs.iter() {
            if xi == i {
                if containers.is_empty() {
                    n1 = 1.0;
                } else {
                    n1 = containers.last().unwrap().get_material().refractive_index;
                }
            }
            if let Some(index) = containers.iter().position(|x| x == &xi.object) {
                containers.remove(index);
            } else {
                containers.push(xi.object.clone())
            }

            if xi == i {
                if containers.is_empty() {
                    n2 = 1.0;
                } else {
                    n2 = containers.last().unwrap().get_material().refractive_index;
                }
                break;
            }
        }

        Self {
            _t: i.t,
            object: i.object.clone(),
            _point: p,
            eye_vec: e,
            normal,
            _inside: inside,
            over_point: p + (normal * EPSILON),
            under_point: p - (normal * EPSILON),
            reflect_vec: r.direction.reflect(&normal),
            n1: n1,
            n2: n2,
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

    pub fn get_reflect_vec(&self) -> &Vec4 {
        &self.reflect_vec
    }

    pub fn get_material(&self) -> &Material {
        self.object.get_material()
    }

    pub fn get_snells_law_value(&self) -> f64 {
        let n_ratio = self.n1 / self.n2;
        let cos_i = self.eye_vec.dot(&self.normal);
        n_ratio.powi(2) * (1.0 - cos_i.powi(2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intersection::Intersections;
    use crate::math::matrix::Mat4;
    use crate::math::SQRT_2;
    use crate::shapes::{Plane, Sphere};

    #[test]
    fn precomputed() {
        let r = Ray::new(&Vec4::point(0.0, 0.0, -5.0), &Vec4::vec(0.0, 0.0, 1.0));

        let s = Sphere::default_boxed();
        let i = Intersection::new(s, 4.0);
        let comps = i.precomputed(&r, None);
        assert_eq!(comps._t, i.t);
        assert_eq!(&comps.object, &i.object);
        assert_eq!(comps._point, Vec4::point(0.0, 0.0, -1.0));
        assert_eq!(comps.eye_vec, Vec4::vec(0.0, 0.0, -1.0));
        assert_eq!(comps.normal, Vec4::vec(0.0, 0.0, -1.0));
        assert_eq!(comps._inside, false);

        let r = Ray::new(&Vec4::POINT_ZERO, &Vec4::vec(0.0, 0.0, 1.0));

        let s = Sphere::default_boxed();
        let i = Intersection::new(s, 1.0);
        let comps = i.precomputed(&r, None);
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
        let comps = i.precomputed(&r, None);
        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps._point.z > comps.over_point.z);
    }

    #[test]
    fn reflection() {
        let plane = Plane::default_boxed();
        let ray = Ray::new(
            &Vec4::point(0.0, 1.0, -1.0),
            &Vec4::vec(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );
        let comps = Intersection::new(plane, SQRT_2).precomputed(&ray, None);
        assert_eq!(
            comps.reflect_vec,
            Vec4::vec(0.0, SQRT_2 / 2.0, SQRT_2 / 2.0)
        );
    }

    #[test]
    fn refractive_indeces() {
        let a = Sphere::new_boxed(Some(Mat4::scaling(2.0, 2.0, 2.0)), Some(Material::GLASS));
        let mut m = Material::GLASS;
        m.refractive_index = 2.0;
        let b = Sphere::new_boxed(Some(Mat4::translation(0.0, 0.0, -0.25)), Some(m.clone()));
        m.refractive_index = 2.5;
        let c = Sphere::new_boxed(Some(Mat4::translation(0.0, 0.0, 0.25)), Some(m));

        let ray = Ray::new(&Vec4::point(0.0, 0.0, -4.0), &Vec4::VEC_Z_ONE);
        let i = vec![
            Intersection::new(a.clone(), 2.0),
            Intersection::new(b.clone(), 2.75),
            Intersection::new(c.clone(), 3.25),
            Intersection::new(b, 4.75),
            Intersection::new(c, 5.25),
            Intersection::new(a, 6.0),
        ];
        let xs = Intersections::from(i);
        let exp_n1 = vec![1.0, 1.5, 2.0, 2.5, 2.5, 1.5];
        let exp_n2 = vec![1.5, 2.0, 2.5, 2.5, 1.5, 1.0];
        for i in 0..xs.len() {
            let comps = xs[i].precomputed(&ray, Some(xs.get_inner_ref()));
            assert_eq!(comps.n1, exp_n1[i], "N1 at case {}", i);
            assert_eq!(comps.n2, exp_n2[i], "N2 ar case {}", i);
        }
    }

    #[test]
    fn underpoint() {
        let r = Ray::new(&Vec4::point(0.0, 0.0, -5.0), &Vec4::VEC_Z_ONE);
        let s = Sphere::new_boxed(
            Some(Mat4::translation(0.0, 0.0, 1.0)),
            Some(Material::GLASS),
        );

        let i = Intersection::new(s, 5.0);
        let xs = Intersections::from(vec![i.clone()]);

        let comps = i.precomputed(&r, Some(xs.get_inner_ref()));
        assert!(comps.under_point.z > EPSILON / 2.0);
        assert!(comps._point.z < comps.under_point.z)
    }
}
