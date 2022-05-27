use crate::color::Color;
use crate::vec4::Vec4;

#[derive(Debug)]
pub struct PointLight {
    pub intensity: Color,
    pub position: Vec4,
}

impl PointLight {
    pub fn new(intensity: Color, position: Vec4) -> Self {
        Self {
            intensity,
            position,
        }
    }
}

impl Default for PointLight {
    fn default() -> Self {
        Self {
            intensity: Color::rgb(1.0, 1.0, 1.0),
            position: Vec4::POINT_ZERO,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let light = PointLight::default();
        assert_eq!(Vec4::POINT_ZERO, light.position);
        assert_eq!(Color::rgb(1.0, 1.0, 1.0), light.intensity);
    }
}
