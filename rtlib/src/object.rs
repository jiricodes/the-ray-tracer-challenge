use crate::intersection::Intersections;
use crate::material::Material;
use crate::math::matrix::Mat4;
use crate::math::vec4::Vec4;
use crate::ray::Ray;

pub trait Object {
    fn with_material(material: Material) -> Self;

    fn intersect<'a>(&'a self, ray: &Ray) -> Intersections;

    fn transform(&mut self, m: &Mat4);

    fn normal_at(&self, p: &Vec4) -> Vec4;
}
