use crate::matrix::Mat4;
use crate::ray::Ray;

pub struct Camera {
    height: u32,
    width: u32,
    fov: f32,
    transform: Mat4,
    pixel_size: f32,
    half_height: f32,
    half_width: f32,
}

impl Camera {
    pub fn new(width: u32, height: u32, fov: f32) -> Self {
        let half_view = (fov / 2.0).tan();
        let aspect = width as f32 / height as f32;
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
            pixel_size: (half_width * 2.0) / width as f32,
            half_height,
            half_width,
        }
    }

    pub fn ray_for_pixel(x: u32, y: u32) -> Ray {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn basic() {
        let camera = Camera::new(160, 120, PI / 2.0);
        assert_eq!(camera.height, 120);
        assert_eq!(camera.width, 160);
        assert_eq!(camera.fov, PI / 2.0);
        assert_eq!(camera.transform, Mat4::IDENTITY);

        let camera = Camera::new(200, 125, PI / 2.0);
        assert_eq!(camera.pixel_size, 0.01);

        let camera = Camera::new(125, 200, PI / 2.0);
        assert_eq!(camera.pixel_size, 0.01);
    }
}
