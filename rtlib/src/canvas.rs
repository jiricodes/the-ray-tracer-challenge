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

    pub fn into_ppm_string(&self) -> String {
        let mut ret = String::new();
        // Header
        ret += "P3\n";
        ret += &format!("{} {}\n", self.width, self.height);
        ret += "255\n";
        // Pixel data
        // R G B per pixel in range 0-255, clamped otherwise
        // max 70 chars per line
        let mut line_len = 0;
        for pixel in self.data.iter() {
            let c = pixel.as_ppm_string();
            if line_len + c.len() + 1 > 70 {
                ret += &format!("\n{}", c);
                line_len = c.len();
            } else if line_len == 0 {
                ret += &c;
                line_len = c.len();
            } else {
                ret += &format!(" {}", c);
                line_len += c.len() + 1;
            }
        }
        ret
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

    #[test]
    fn get_ppm() {
        let mut canvas = Canvas::new(10, 20);
        canvas.data[105] = Color::WHITE;
        let ppm = canvas.into_ppm_string();
        for (i, line) in ppm.lines().enumerate() {
            match i {
                0 => assert_eq!(line, "P3"),
                1 => assert_eq!(line, "10 20"),
                2 => assert_eq!(line, "255"),
                _ => assert!(line.len() <= 70),
            }
        }
    }
}
