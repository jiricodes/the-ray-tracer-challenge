use crate::sphere::Sphere;

pub struct Intersection<'a> {
    object: &'a Sphere,
    t: f32,
}

impl<'a> Intersection<'a> {
    pub fn new(object: &'a Sphere, t: f32) -> Self {
        Self { object, t }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let s = Sphere::new();
        let i = Intersection::new(&s, 3.5);
        assert_eq!(i.object, &s);
        assert_eq!(i.t, 3.5);
    }
}
