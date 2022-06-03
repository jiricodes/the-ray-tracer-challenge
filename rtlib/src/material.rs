use crate::color::Color;
use crate::light::PointLight;
use crate::math::vec4::Vec4;
use crate::patterns::BoxPattern;
use crate::shapes::Shape;

#[derive(Debug, PartialEq, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub pattern: Option<BoxPattern>,
}

impl Material {
    pub fn new(
        color: Color,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
        pattern: Option<BoxPattern>,
    ) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
            pattern,
        }
    }

    pub fn lighting(
        &self,
        object: &dyn Shape,
        p: &Vec4,
        light: &PointLight,
        eye_vec: &Vec4,
        normal: &Vec4,
        in_snadow: bool,
    ) -> Color {
        let color = match &self.pattern {
            Some(pat) => pat.pattern_at(object, *p),
            None => self.color,
        };

        // combine the surface color with the light's color intensity
        let eff_color = color * light.intensity;

        // Direction to the light source
        let light_dir = (light.position - p).normalize();

        // Ambient contribution
        let ambient = eff_color * self.ambient;

        // if in shadow, then ignore diffuse and specular
        if in_snadow {
            return ambient;
        }

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
            pattern: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::patterns;
    use crate::shapes::Sphere;

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
        let s = Sphere::default();

        // Light behind the eye
        let eye_vec = Vec4::new_vec(0.0, 0.0, -1.0);
        let normal = Vec4::new_vec(0.0, 0.0, -1.0);
        let light = PointLight::new(Vec4::new_point(0.0, 0.0, -10.0), Color::rgb(1.0, 1.0, 1.0));
        assert_eq!(
            Color::rgb(1.9, 1.9, 1.9),
            m.lighting(&s, &p, &light, &eye_vec, &normal, false)
        );

        // Eye PI/2 off normal
        let eye_vec = Vec4::new_vec(0.0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
        let normal = Vec4::new_vec(0.0, 0.0, -1.0);
        let light = PointLight::new(Vec4::new_point(0.0, 0.0, -10.0), Color::rgb(1.0, 1.0, 1.0));
        assert_eq!(
            Color::rgb(1.0, 1.0, 1.0),
            m.lighting(&s, &p, &light, &eye_vec, &normal, false)
        );

        // Eye on normal, light PI/2 offsets
        let eye_vec = Vec4::new_vec(0.0, 0.0, -1.0);
        let normal = Vec4::new_vec(0.0, 0.0, -1.0);
        let light = PointLight::new(Vec4::new_point(0.0, 10.0, -10.0), Color::rgb(1.0, 1.0, 1.0));
        let c = 0.1 + 0.9 * 2f64.sqrt() / 2.0 + 0.0; // 0.7364
        assert_eq!(
            Color::rgb(c, c, c),
            m.lighting(&s, &p, &light, &eye_vec, &normal, false)
        );

        // Light PI/2 offsets, eye directly on reflection path
        let eye_vec = Vec4::new_vec(0.0, -2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
        let normal = Vec4::new_vec(0.0, 0.0, -1.0);
        let light = PointLight::new(Vec4::new_point(0.0, 10.0, -10.0), Color::rgb(1.0, 1.0, 1.0));
        let c = 1.6363961030678928; // 0.1 + 0.9 * 2f64.sqrt() / 2.0 + 0.9; // 1.6364
        assert_eq!(
            Color::rgb(c, c, c),
            m.lighting(&s, &p, &light, &eye_vec, &normal, false)
        );

        // Light behind the surface
        let eye_vec = Vec4::new_vec(0.0, 0.0, -1.0);
        let normal = Vec4::new_vec(0.0, 0.0, -1.0);
        let light = PointLight::new(Vec4::new_point(0.0, 0.0, 10.0), Color::rgb(1.0, 1.0, 1.0));
        assert_eq!(
            Color::rgb(0.1, 0.1, 0.1),
            m.lighting(&s, &p, &light, &eye_vec, &normal, false)
        );

        // Lighting with the surface in shadow
        let eye_vec = Vec4::new_vec(0.0, 0.0, -1.0);
        let normal = Vec4::new_vec(0.0, 0.0, -1.0);
        let light = PointLight::new(Vec4::new_point(0.0, 0.0, -10.0), Color::rgb(1.0, 1.0, 1.0));
        assert_eq!(
            Color::rgb(0.1, 0.1, 0.1),
            m.lighting(&s, &p, &light, &eye_vec, &normal, true)
        );
    }

    #[test]
    fn pattern_lighting() {
        let mut m = Material::default();
        m.pattern = Some(patterns::StripePattern::default_boxed());
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;
        let object = Sphere::default();
        let eye_vec = Vec4::new_vec(0.0, 0.0, -1.0);
        let normal = Vec4::new_vec(0.0, 0.0, -1.0);
        let light = PointLight::new(Vec4::new_point(0.0, 0.0, -10.0), Color::WHITE);
        let c1 = m.lighting(
            &object,
            &Vec4::new_point(0.9, 0.0, 0.0),
            &light,
            &eye_vec,
            &normal,
            false,
        );
        let c2 = m.lighting(
            &object,
            &Vec4::new_point(1.1, 0.0, 0.0),
            &light,
            &eye_vec,
            &normal,
            false,
        );

        assert_eq!(c1, Color::WHITE);
        assert_eq!(c2, Color::BLACK);
    }
}
