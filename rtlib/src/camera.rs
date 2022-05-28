use crate::matrix::Mat4;

pub struct Camera {
    height: u32,
    width: u32,
    fov: f32,
    transform: Mat4,
}

impl Camera {
    pub fn new(height: u32, width: u32, fov: f32) -> Self {
        Self {
            height,
            width,
            fov,
            transform: Mat4::IDENTITY,
        }
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn basic() {
		let camera = Camera::new(160, 120)
	}
}