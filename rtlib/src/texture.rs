use crate::color::Color;
use crate::math::vec4::Vec4;

pub struct Material {}

pub struct Pattern {
    colors: Vec<Color>,
}

impl Pattern {
    pub fn new(colors: Vec<Color>) -> Self {
        Self { colors }
    }

    pub fn stripe_at(&self, point: &Vec4) -> Color {
        let i = point.x.floor().abs() as usize % self.colors.len();
        self.colors[i]
    }
}

impl Default for Pattern {
    fn default() -> Self {
        Self {
            colors: vec![Color::WHITE, Color::BLACK],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let pattern = Pattern::default();
        assert_eq!(pattern.colors[0], Color::WHITE);
        assert_eq!(pattern.colors[1], Color::BLACK);
    }

    #[test]
    fn stripes_y_axis() {
        let pattern = Pattern::default();
        assert_eq!(pattern.stripe_at(&Vec4::POINT_ZERO), Color::WHITE);
        assert_eq!(
            pattern.stripe_at(&Vec4::new_point(0.0, 1.0, 0.0)),
            Color::WHITE
        );
        assert_eq!(
            pattern.stripe_at(&Vec4::new_point(0.0, 2.0, 0.0)),
            Color::WHITE
        );
    }

    #[test]
    fn stripes_z_axis() {
        let pattern = Pattern::default();
        assert_eq!(pattern.stripe_at(&Vec4::POINT_ZERO), Color::WHITE);
        assert_eq!(
            pattern.stripe_at(&Vec4::new_point(0.0, 0.0, 1.0)),
            Color::WHITE
        );
        assert_eq!(
            pattern.stripe_at(&Vec4::new_point(0.0, 0.0, 2.0)),
            Color::WHITE
        );
    }

    #[test]
    fn stripes_alter() {
        let pattern = Pattern::default();
        assert_eq!(pattern.stripe_at(&Vec4::POINT_ZERO), Color::WHITE);
        assert_eq!(
            pattern.stripe_at(&Vec4::new_point(0.9, 1.0, 0.0)),
            Color::WHITE
        );
        assert_eq!(
            pattern.stripe_at(&Vec4::new_point(1.0, 0.0, 0.0)),
            Color::BLACK
        );
        assert_eq!(
            pattern.stripe_at(&Vec4::new_point(-0.1, 0.0, 0.0)),
            Color::BLACK
        );
        assert_eq!(
            pattern.stripe_at(&Vec4::new_point(-1.0, 0.0, 0.0)),
            Color::BLACK
        );
        assert_eq!(
            pattern.stripe_at(&Vec4::new_point(-1.1, 0.0, 0.0)),
            Color::WHITE
        );
    }
}
