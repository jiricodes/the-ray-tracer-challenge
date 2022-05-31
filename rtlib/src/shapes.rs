pub mod sphere;

use crate::intersection::Intersections;
use crate::material::Material;
use crate::math::matrix::Mat4;
use crate::math::vec4::Vec4;
use crate::ray::Ray;

use std::any::Any;
use std::fmt::Debug;

pub trait Shape: Any + Debug {
    fn as_any(&self) -> &dyn Any;
    fn box_clone(&self) -> BoxShape;
    fn box_eq(&self, other: &dyn Any) -> bool;

    fn set_material(&mut self, material: Material);
    fn get_material(&self) -> &Material;

    fn local_intersect(&self, local_ray: Ray) -> Intersections;
    fn local_normal_at(&self, local_point: Vec4) -> Vec4;

    fn transform(&mut self, m: &Mat4);
    fn transformation(&self) -> &Mat4;
    fn inverse_transformation(&self) -> &Mat4;

    fn intersect(&self, world_ray: &Ray) -> Intersections {
        self.local_intersect(world_ray.transform(self.inverse_transformation()))
    }

    fn normal_at(&self, world_point: Vec4) -> Vec4 {
        let object_normal = self.local_normal_at(self.inverse_transformation() * world_point);
        let mut world_normal = self.inverse_transformation() * object_normal;
        world_normal.w = 0.0;
        world_normal
    }
}

pub type BoxShape = Box<dyn Shape>;