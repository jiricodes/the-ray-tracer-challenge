use crate::color::Color;
use crate::light::PointLight;
use crate::math::vec4::Vec4;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Material {
    pub fn new(color: Color, ambient: f32, diffuse: f32, specular: f32, shininess: f32) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn lighting(&self, p: &Vec4, light: &PointLight, eye_vec: &Vec4, normal: &Vec4) -> Color {
        // combine the surface color with the light's color intensity
        let eff_color = self.color * light.intensity;

        // Direction to the light source
        let light_dir = (light.position - p).normalize();

        // Ambient contribution
        let ambient = eff_color * self.ambient;

        // check if the light is behind the surface
        let light_dot_normal = light_dir.dot(normal);

        let (diffuse, specular) = if light_dot_normal < 0.0 {
            // the light is behind the surface, so diffuse and specular are black
            (Color::BLACK, Color::BLACK)
        } else {
            // get the diffuse
            let dif = eff_color * self.diffuse * light_dot_normal;

            // check reflection direction relative to the eye
            let reflect_dir = (-light_dir).reflect(normal);
            let reflect_dot_eye = reflect_dir.dot(eye_vec);

            let spec = if reflect_dot_eye < 0.0 {
                Color::BLACK
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                light.intensity * self.specular * factor
            };
            (dif, spec)
        };
        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let m = Material::default();

        assert_eq!(Color::WHITE, m.color);
        assert_eq!(0.1, m.ambient);
        assert_eq!(0.9, m.diffuse);
        assert_eq!(0.9, m.specular);
        assert_eq!(200.0, m.shininess);
    }

    #[test]
    fn lighting() {
        let m = Material::default();
        let p = Vec4::POINT_ZERO;

        // Light behind the eye
        let eye_vec = Vec4::new_vec(0.0, 0.0, -1.0);
        let normal = Vec4::new_vec(0.0, 0.0, -1.0);
        let light = PointLight::new(Vec4::new_point(0.0, 0.0, -10.0), Color::rgb(1.0, 1.0, 1.0));
        assert_eq!(
            Color::rgb(1.9, 1.9, 1.9),
            m.lighting(&p, &light, &eye_vec, &normal)
        );

        // Eye PI/2 off normal
        let eye_vec = Vec4::new_vec(0.0, 2f32.sqrt() / 2.0, -2f32.sqrt() / 2.0);
        let normal = Vec4::new_vec(0.0, 0.0, -1.0);
        let light = PointLight::new(Vec4::new_point(0.0, 0.0, -10.0), Color::rgb(1.0, 1.0, 1.0));
        assert_eq!(
            Color::rgb(1.0, 1.0, 1.0),
            m.lighting(&p, &light, &eye_vec, &normal)
        );

        // Eye on normal, light PI/2 offsets
        let eye_vec = Vec4::new_vec(0.0, 0.0, -1.0);
        let normal = Vec4::new_vec(0.0, 0.0, -1.0);
        let light = PointLight::new(Vec4::new_point(0.0, 10.0, -10.0), Color::rgb(1.0, 1.0, 1.0));
        let c = 0.1 + 0.9 * 2f32.sqrt() / 2.0 + 0.0; // 0.7364
        assert_eq!(
            Color::rgb(c, c, c),
            m.lighting(&p, &light, &eye_vec, &normal)
        );

        // Light PI/2 offsets, eye directly on reflection path
        let eye_vec = Vec4::new_vec(0.0, -2f32.sqrt() / 2.0, -2f32.sqrt() / 2.0);
        let normal = Vec4::new_vec(0.0, 0.0, -1.0);
        let light = PointLight::new(Vec4::new_point(0.0, 10.0, -10.0), Color::rgb(1.0, 1.0, 1.0));
        let c = 1.6363853; // 0.1 + 0.9 * 2f32.sqrt() / 2.0 + 0.9; // 1.6364
        assert_eq!(
            Color::rgb(c, c, c),
            m.lighting(&p, &light, &eye_vec, &normal)
        );

        // Light behind the surface
        let eye_vec = Vec4::new_vec(0.0, 0.0, -1.0);
        let normal = Vec4::new_vec(0.0, 0.0, -1.0);
        let light = PointLight::new(Vec4::new_point(0.0, 0.0, 10.0), Color::rgb(1.0, 1.0, 1.0));
        assert_eq!(
            Color::rgb(0.1, 0.1, 0.1),
            m.lighting(&p, &light, &eye_vec, &normal)
        );
    }
}
