pub mod sphere;
pub use sphere::Sphere;

pub mod plane;
pub use plane::Plane;

pub mod cube;
pub use cube::Cube;

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
    fn set_transform(&mut self, transformation: Mat4);
    fn transformation(&self) -> &Mat4;
    fn inverse_transformation(&self) -> &Mat4;

    fn intersect(&self, world_ray: &Ray) -> Intersections {
        self.local_intersect(world_ray.transform(self.inverse_transformation()))
    }

    fn normal_at(&self, world_point: Vec4) -> Vec4 {
        let object_normal = self.local_normal_at(self.inverse_transformation() * world_point);
        let mut world_normal = self.inverse_transformation().transpose() * object_normal;
        world_normal.w = 0.0;
        world_normal.normalize()
    }
}

pub type BoxShape = Box<dyn Shape>;

impl Clone for BoxShape {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl PartialEq for BoxShape {
    fn eq(&self, other: &BoxShape) -> bool {
        self.box_eq(other.as_any())
    }
}

#[cfg(test)]
mod testshape {
    use super::*;
    use crate::intersection::Intersection;
    use crate::math::EPSILON;
    use crate::util::uid;

    #[derive(Debug, Clone)]
    pub struct TestShape {
        uid: usize,
        pub transform: Mat4,
        pub inverse_transform: Mat4,
        pub material: Material,
    }

    impl PartialEq for TestShape {
        fn eq(&self, other: &Self) -> bool {
            self.uid == other.uid
                && self.transform == other.transform
                && self.material == other.material
        }
    }

    impl Shape for TestShape {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn box_clone(&self) -> BoxShape {
            Box::new((*self).clone())
        }
        fn box_eq(&self, other: &dyn Any) -> bool {
            other.downcast_ref::<Self>().map_or(false, |a| self == a)
        }

        fn set_material(&mut self, material: Material) {
            self.material = material;
        }
        fn get_material(&self) -> &Material {
            &self.material
        }

        fn transform(&mut self, m: &Mat4) {
            self.transform = m * self.transform;
        }
        fn set_transform(&mut self, transform: Mat4) {
            self.transform = transform;
            self.inverse_transform = self.transform.inverse().unwrap();
        }
        fn transformation(&self) -> &Mat4 {
            &self.transform
        }
        fn inverse_transformation(&self) -> &Mat4 {
            &self.inverse_transform
        }

        fn local_normal_at(&self, _local_point: Vec4) -> Vec4 {
            unimplemented!()
        }
        fn local_intersect(&self, _local_ray: Ray) -> Intersections {
            unimplemented!()
        }
    }

    impl TestShape {
        pub fn new(transform: Option<Mat4>, material: Option<Material>) -> Self {
            let transform = transform.unwrap_or_default();
            let inverse_transform = transform.inverse().unwrap();
            Self {
                uid: uid::fetch_uid(),
                transform,
                material: material.unwrap_or_default(),
                inverse_transform,
            }
        }

        pub fn new_boxed(transform: Option<Mat4>, material: Option<Material>) -> BoxShape {
            Box::new(Self::new(transform, material))
        }

        pub fn default_boxed() -> BoxShape {
            Box::new(Self::default())
        }
    }

    impl Default for TestShape {
        fn default() -> Self {
            Self {
                uid: uid::fetch_uid(),
                transform: Mat4::default(),
                inverse_transform: Mat4::default(),
                material: Material::default(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testshape_basic() {
        let object = testshape::TestShape::default();
        assert_eq!(object.transform, Mat4::IDENTITY);
        assert_eq!(object.material, Material::default());
    }

    #[test]
    fn testshape_transformed() {
        let object = testshape::TestShape::new(Some(Mat4::translation(2.0, 3.0, 4.0)), None);
        assert_eq!(object.transform, Mat4::translation(2.0, 3.0, 4.0));
    }

    #[test]
    fn testshape_materialed() {
        let mut m = Material::default();
        m.ambient = 1.0;
        let object = testshape::TestShape::new(None, Some(m.clone()));
        assert_eq!(object.material, m);
    }
}
