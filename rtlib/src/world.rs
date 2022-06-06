use crate::color::Color;
use crate::intersection::Intersections;
use crate::light::PointLight;
use crate::material::Material;
use crate::math::matrix::Mat4;
use crate::math::vec4::Vec4;
use crate::precompute::PreCompute;
use crate::ray::Ray;
use crate::shapes::sphere::Sphere;
use crate::shapes::BoxShape;

pub struct World {
    pub objects: Vec<BoxShape>,
    pub lights: Vec<PointLight>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: BoxShape) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let mut intersections = Intersections::new();
        for object in self.objects.iter() {
            let mut ints = object.intersect(ray);
            intersections.append(&mut ints);
        }
        intersections.sort();
        intersections
    }

    pub fn shade_hit(&self, comps: &PreCompute, max_reflections: u32) -> Color {
        let mut color = Color::BLACK;
        let is_shadowed = self.is_shadowed(comps.get_overpoint());
        for light in self.lights.iter() {
            color = color + comps.lighting(light, is_shadowed);
        }
        color + self.reflected_color(comps, max_reflections)
    }

    pub fn color_at(&self, r: &Ray, max_reflections: u32) -> Color {
        let xs = self.intersect(r);
        if let Some(i) = xs.hit() {
            let comps = PreCompute::new(&i, r);
            self.shade_hit(&comps, max_reflections)
        } else {
            Color::BLACK
        }
    }

    pub fn is_shadowed(&self, p: &Vec4) -> bool {
        for light in self.lights.iter() {
            let p_to_l = light.position - p;
            let light_dist = p_to_l.magnitude();
            let ray = Ray::new(p, &p_to_l.normalize());
            let xs = self.intersect(&ray);
            let h = xs.hit();
            if h.is_some() && h.unwrap().t < light_dist {
                return true;
            }
        }
        false
    }

    pub fn reflected_color(&self, comps: &PreCompute, max_reflections: u32) -> Color {
        let reflectivness = comps.get_material().reflectivness;
        if reflectivness <= 0.0 || max_reflections <= 0 {
            return Color::BLACK;
        }
        let reflect_ray = Ray::new(comps.get_overpoint(), comps.get_reflect_vec());
        let color = self.color_at(&reflect_ray, max_reflections - 1);
        color * reflectivness
    }
}

impl Default for World {
    fn default() -> Self {
        let mut w = Self::new();
        // default light
        let light = PointLight::new(Vec4::point(-10.0, 10.0, -10.0), Color::WHITE);
        w.add_light(light);

        // Default sphere 1
        let mut material = Material::default();
        material.color = Color::rgb(0.8, 1.0, 0.6);
        material.diffuse = 0.7;
        material.specular = 0.2;

        let s = Sphere::new_boxed(None, Some(material));
        w.add_object(s);

        // Default sphere 2
        let mut s = Sphere::default_boxed();
        s.set_transform(Mat4::scaling(0.5, 0.5, 0.5));
        w.add_object(s);
        w
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intersection::Intersection;
    use crate::math::SQRT_2;
    use crate::shapes::Plane;

    #[test]
    fn basic() {
        let w = World::new();
        assert!(w.objects.is_empty());
        assert!(w.lights.is_empty());

        let w = World::default();
        assert_eq!(2, w.objects.len());
        assert_eq!(Color::rgb(0.8, 1.0, 0.6), w.objects[0].get_material().color);
        assert_eq!(0.7, w.objects[0].get_material().diffuse);
        assert_eq!(0.2, w.objects[0].get_material().specular);

        assert_eq!(Mat4::scaling(0.5, 0.5, 0.5), *w.objects[1].transformation());

        assert_eq!(1, w.lights.len());
    }

    #[test]
    fn intersect() {
        let w = World::default();
        let r = Ray::new(&Vec4::point(0.0, 0.0, -5.0), &Vec4::vec(0.0, 0.0, 1.0));
        let mut xs = w.intersect(&r);
        xs.sort();
        assert_eq!(4, xs.len());
        assert_eq!(4.0, xs[0].t);
        assert_eq!(4.5, xs[1].t);
        assert_eq!(5.5, xs[2].t);
        assert_eq!(6.0, xs[3].t);
    }

    #[test]
    fn hit_shading() {
        // Outside intersection
        let mut w = World::default();
        let r = Ray::new(&Vec4::point(0.0, 0.0, -5.0), &Vec4::VEC_Z_ONE);
        let i = Intersection::new(w.objects[0].clone(), 4.0);
        let comps = PreCompute::new(&i, &r);
        let color = w.shade_hit(&comps, 0);
        assert_eq!(Color::rgb(0.38066, 0.47583, 0.2855), color);

        //Inside intersection
        w.lights[0] = PointLight::new(Vec4::point(0.0, 0.25, 0.0), Color::WHITE);
        let r = Ray::new(&Vec4::POINT_ZERO, &Vec4::VEC_Z_ONE);
        let i = Intersection::new(w.objects[1].clone(), 0.5);
        let comps = PreCompute::new(&i, &r);
        let color = w.shade_hit(&comps, 0);
        assert_eq!(Color::rgb(0.90498, 0.90498, 0.90498), color);
    }

    #[test]
    fn ball_in_shadows() {
        let mut w = World::new();
        w.add_light(PointLight::new(
            Vec4::point(0.0, 0.0, -10.0),
            Color::rgb(1.0, 1.0, 1.0),
        ));
        w.add_object(Sphere::default_boxed());
        let mut s = Sphere::default_boxed();
        s.set_transform(Mat4::translation(0.0, 0.0, 10.0));
        w.add_object(s);
        let ray = Ray::new(&Vec4::point(0.0, 0.0, 5.0), &Vec4::VEC_Z_ONE);
        let i = Intersection::new(w.objects[1].clone(), 4.0);
        let comps = PreCompute::new(&i, &ray);
        assert_eq!(Color::rgb(0.1, 0.1, 0.1), w.shade_hit(&comps, 0));
    }

    #[test]
    fn color_at() {
        let mut w = World::default();

        // No hit
        let r = Ray::new(&Vec4::point(0.0, 0.0, -5.0), &Vec4::VEC_Y_ONE);
        let color = w.color_at(&r, 0);
        assert_eq!(Color::BLACK, color);

        // Default hit
        let r = Ray::new(&Vec4::point(0.0, 0.0, -5.0), &Vec4::VEC_Z_ONE);
        let color = w.color_at(&r, 0);
        assert_eq!(Color::rgb(0.38066, 0.47583, 0.2855), color);

        // hit small from inside of big
        let mut m = Material::default();
        m.ambient = 1.0;
        w.objects[0].set_material(m.clone());
        w.objects[1].set_material(m);
        let r = Ray::new(&Vec4::point(0.0, 0.0, 0.75), &-Vec4::VEC_Z_ONE);
        let color = w.color_at(&r, 0);
        assert_eq!(w.objects[1].get_material().color, color);
    }

    #[test]
    fn shadows() {
        let w = World::default();
        let p = Vec4::point(0.0, 10.0, 0.0);
        assert_eq!(false, w.is_shadowed(&p));

        let p = Vec4::point(10.0, -10.0, 10.0);
        assert_eq!(true, w.is_shadowed(&p));

        let p = Vec4::point(-2.0, 2.0, -2.0);
        assert_eq!(false, w.is_shadowed(&p));
    }

    #[test]
    fn reflect_non_reflective() {
        let mut w = World::default();
        let r = Ray::new(&Vec4::POINT_ZERO, &Vec4::VEC_Y_ONE);
        let mut m = Material::default();
        m.ambient = 1.0;
        w.objects[1].set_material(m);
        let comps = Intersection::new(w.objects[1].clone(), 1.0).precomputed(&r);
        assert_eq!(w.reflected_color(&comps, 5), Color::BLACK);
    }

    #[test]
    fn reflect_reflective() {
        let mut w = World::default();

        let plane = Plane::new_boxed(
            Some(Mat4::translation(0.0, -1.0, 0.0)),
            Some(Material {
                reflectivness: 0.5,
                ..Default::default()
            }),
        );

        w.add_object(plane.clone());

        let r = Ray::new(
            &Vec4::point(0.0, 0.0, -3.0),
            &Vec4::vec(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );

        let comps = Intersection::new(plane, SQRT_2).precomputed(&r);
        assert_eq!(
            w.reflected_color(&comps, 5),
            Color::rgb(0.190332201495133, 0.23791525186891627, 0.14274915112134975)
        );
    }

    #[test]
    fn shade_hit_reflective() {
        let mut w = World::default();

        let plane = Plane::new_boxed(
            Some(Mat4::translation(0.0, -1.0, 0.0)),
            Some(Material {
                reflectivness: 0.5,
                ..Default::default()
            }),
        );

        w.add_object(plane.clone());

        let r = Ray::new(
            &Vec4::point(0.0, 0.0, -3.0),
            &Vec4::vec(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );

        let comps = Intersection::new(plane, SQRT_2).precomputed(&r);
        assert_eq!(
            w.shade_hit(&comps, 5),
            Color::rgb(0.8767572837020907, 0.924340334075874, 0.8291742333283075)
        );
    }

    #[test]
    fn infinite_reflection() {
        let mut w = World::new();
        let l = PointLight::default();

        let lower_plane = Plane::new_boxed(
            Some(Mat4::translation(0.0, -1.0, 0.0)),
            Some(Material {
                reflectivness: 0.5,
                ..Default::default()
            }),
        );

        let upper_plane = Plane::new_boxed(
            Some(Mat4::translation(0.0, -1.0, 0.0)),
            Some(Material {
                reflectivness: 0.5,
                ..Default::default()
            }),
        );

        w.add_light(l);
        w.add_object(lower_plane);
        w.add_object(upper_plane);
        let ray = Ray::new(&Vec4::POINT_ZERO, &Vec4::VEC_Y_ONE);
        let _ = w.color_at(&ray, 5);
    }
}
