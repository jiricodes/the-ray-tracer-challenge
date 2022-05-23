use crate::color::Color;

#[derive(Debug, Clone)]
pub struct Canvas {
    width: u32,
    height: u32,
    data: Vec<Color>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![Color::BLACK; width as usize * height as usize],
        }
    }

    fn in_bounds(&self, x: u32, y: u32) {
        if x >= self.width && y >= self.height {
            panic!("Point out of canvas bounds");
        }
    }

    fn index(&self, x: u32, y: u32) -> usize {
        (y * self.height + x) as usize
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.in_bounds(x, y);
        let i = self.index(x, y);
        self.data[i] = color;
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        self.in_bounds(x, y);
        let i = self.index(x, y);
        self.data[i]
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            data: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        assert_eq!(canvas.data.len(), 10 * 20);
        assert!(canvas.data.iter().all(|x| *x == Color::BLACK));
    }
    #[test]
    fn put_pix() {
        let mut canvas = Canvas::new(10, 20);
        canvas.put_pixel(5, 5, Color::WHITE);
        assert_eq!(canvas.data[105], Color::WHITE);
    }
    #[test]
    fn get_pix() {
        let mut canvas = Canvas::new(10, 20);
        canvas.data[105] = Color::WHITE;
        assert_eq!(canvas.get_pixel(5, 5), Color::WHITE);
    }
}
