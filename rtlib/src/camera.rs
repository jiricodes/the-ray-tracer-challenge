use crate::matrix::Mat4;
use crate::ray::Ray;
use crate::vec4::Vec4;

pub struct Camera {
    height: u32,
    width: u32,
    fov: f64,
    transform: Mat4,
    pixel_size: f64,
    half_height: f64,
    half_width: f64,
}

impl Camera {
    pub fn new(width: u32, height: u32, fov: f64) -> Self {
        let half_view = (fov / 2.0).tan();
        let aspect = width as f64 / height as f64;
        let (half_width, half_height) = if aspect < 1.0 {
            (half_view * aspect, half_view)
        } else {
            (half_view, half_view / aspect)
        };
        Self {
            height,
            width,
            fov,
            transform: Mat4::IDENTITY,
            pixel_size: (half_width * 2.0) / width as f64,
            half_height,
            half_width,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn view_transform(&mut self, from: &Vec4, to: &Vec4, up: &Vec4) {
        self.transform = Mat4::view_transform(from, to, up);
    }

    pub fn ray_for_pixel(&self, x: u32, y: u32) -> Ray {
        let x_offset = (x as f64 + 0.5) * self.pixel_size;
        let y_offset = (y as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let pixel = self
            .transform
            .inverse()
            .expect("Camera transform inverse matrix")
            * Vec4::new_point(world_x, world_y, -1.0);
        let origin = self
            .transform
            .inverse()
            .expect("Camera transform inverse matrix")
            * Vec4::POINT_ZERO;
        let direction = (pixel - origin).normalize();
        Ray::new(&origin, &direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::epsilon::EPSILON;
    use std::f64::consts::PI;

    #[test]
    fn basic() {
        let camera = Camera::new(160, 120, PI / 2.0);
        assert_eq!(camera.height, 120);
        assert_eq!(camera.width, 160);
        assert_eq!(camera.fov, PI / 2.0);
        assert_eq!(camera.transform, Mat4::IDENTITY);

        let camera = Camera::new(200, 125, PI / 2.0);
        assert!((camera.pixel_size - 0.01).abs() < EPSILON);

        let camera = Camera::new(125, 200, PI / 2.0);
        assert!((camera.pixel_size - 0.01).abs() < EPSILON);
    }

    #[test]
    fn ray_for_pixel() {
        let mut camera = Camera::new(201, 101, PI / 2.0);

        let r = camera.ray_for_pixel(100, 50);
        assert_eq!(r.origin, Vec4::POINT_ZERO);
        assert_eq!(r.direction, -Vec4::VEC_Z_ONE);

        let r = camera.ray_for_pixel(0, 0);
        assert_eq!(r.origin, Vec4::POINT_ZERO);
        assert_eq!(r.direction, Vec4::new_vec(0.66519, 0.33259, -0.66851));

        camera.transform = Mat4::rotation_y(PI / 4.0) * Mat4::translation(0.0, -2.0, 5.0);
        let r = camera.ray_for_pixel(100, 50);
        assert_eq!(r.origin, Vec4::new_point(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            Vec4::new_vec(2f64.sqrt() / 2.0, 0.0, -2f64.sqrt() / 2.0)
        );
    }
}
