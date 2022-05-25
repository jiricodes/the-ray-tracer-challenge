use crate::vec4::Vec4;

pub struct Ray {
    pub origin: Vec4,
    pub direction: Vec4,
}

impl Ray {
    pub fn new(origin: &Vec4, direction: &Vec4) -> Self {
        Self {
            origin: *origin,
            direction: *direction,
        }
    }

    pub fn position(&self, t: f32) -> Vec4 {
        self.origin + (self.direction * t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let origin = Vec4::new_point(1.0, 2.0, 3.0);
        let direction = Vec4::new_vec(4.0, 5.0, 6.0);
        let ray = Ray::new(&origin, &direction);
        assert_eq!(origin, ray.origin);
        assert_eq!(direction, ray.direction);
    }

    #[test]
    fn position() {
        let r = Ray::new(
            &Vec4::new_point(2.0, 3.0, 4.0),
            &Vec4::new_vec(1.0, 0.0, 0.0),
        );
        assert_eq!(Vec4::new_point(2.0, 3.0, 4.0), r.position(0.0));
        assert_eq!(Vec4::new_point(3.0, 3.0, 4.0), r.position(1.0));
        assert_eq!(Vec4::new_point(1.0, 3.0, 4.0), r.position(-1.0));
        assert_eq!(Vec4::new_point(4.5, 3.0, 4.0), r.position(2.5));
    }
}
