use crate::camera::Camera;
use crate::canvas::Canvas;
use crate::world::World;

pub struct RenderSettings {
    pub recursion_limit: u32,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self { recursion_limit: 5 }
    }
}

pub fn render(camera: &Camera, world: &World, settings: &RenderSettings) -> Canvas {
    let w = camera.get_width();
    let h = camera.get_height();
    let mut image = Canvas::new(w, h);

    for y in 0..h {
        for x in 0..w {
            let r = camera.ray_for_pixel(x, y);
            let color = world.color_at(&r, settings.recursion_limit);
            let _ = image.put_pixel(x, y, color);
        }
    }

    image
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::math::vec4::Vec4;
    use std::f64::consts::PI;

    #[test]
    fn basic_render() {
        let w = World::default();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = Vec4::point(0.0, 0.0, -5.0);
        let to = Vec4::POINT_ZERO;
        let up = Vec4::VEC_Y_ONE;
        c.view_transform(&from, &to, &up);
        let image: Canvas = render(&c, &w, &RenderSettings { recursion_limit: 0 });
        assert_eq!(
            image.get_pixel(5, 5).unwrap(),
            Color::rgb(0.38066, 0.47583, 0.2855)
        )
    }
}
