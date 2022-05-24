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

    fn in_bounds(&self, x: u32, y: u32) -> Result<(), &'static str> {
        if x >= self.width || y >= self.height {
            Err("Point out of canvas bounds")
        } else {
            Ok(())
        }
    }

    fn index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, color: Color) -> Result<(), &'static str> {
        self.in_bounds(x, y)?;
        let i = self.index(x, y);
        self.data[i] = color;
        Ok(())
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Result<Color, &'static str> {
        self.in_bounds(x, y)?;
        let i = self.index(x, y);
        Ok(self.data[i])
    }

    pub fn clear(&mut self, color: Option<Color>) {
        let c = if color.is_none() {
            Color::BLACK
        } else {
            color.unwrap()
        };
        for d in self.data.iter_mut() {
            *d = c;
        }
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
        for (i, pixel) in self.data.iter().enumerate() {
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
            if i as u32 % self.width == self.width - 1 {
                ret += "\n";
                line_len = 0;
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
        let ret = canvas.put_pixel(5, 5, Color::WHITE);
        assert!(ret.is_ok());
        assert_eq!(canvas.data[55], Color::WHITE);
    }
    #[test]
    fn get_pix() {
        let mut canvas = Canvas::new(10, 20);
        canvas.data[105] = Color::WHITE;
        assert_eq!(canvas.get_pixel(5, 10), Ok(Color::WHITE));
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

        canvas = Canvas::new(5, 3);
        let ret = canvas.put_pixel(0, 0, Color::rgb(1.5, 0.0, 0.0));
        assert!(ret.is_ok());
        let ret = canvas.put_pixel(2, 1, Color::rgb(0.0, 0.5, 0.0));
        assert!(ret.is_ok());
        let ret = canvas.put_pixel(4, 2, Color::rgb(-0.5, 0.0, 1.0));
        assert!(ret.is_ok());
        let ppm = canvas.into_ppm_string();
        for (i, line) in ppm.lines().enumerate() {
            match i {
                0 => assert_eq!(line, "P3"),
                1 => assert_eq!(line, "5 3"),
                2 => assert_eq!(line, "255"),
                3 => assert_eq!(line, "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0"),
                4 => assert_eq!(line, "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0"),
                5 => assert_eq!(line, "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"),
                _ => panic!("Unexpected number of lines {}", i),
            }
        }
    }

    #[test]
    fn get_ppm_long_lines() {
        let mut canvas = Canvas::new(10, 2);
        canvas.clear(Some(Color::rgb(1.0, 0.8, 0.6)));
        let ppm = canvas.into_ppm_string();
        for line in ppm.lines() {
            assert!(line.len() <= 70);
        }

        let mut canvas = Canvas::new(11, 2);
        canvas.clear(Some(Color::rgb(1.0, 0.8, 0.6)));
        let ret = canvas.put_pixel(1, 1, Color::BLACK);
        assert!(ret.is_ok());
        let ppm = canvas.into_ppm_string();
        for line in ppm.lines() {
            assert!(line.len() <= 70);
        }
        println!("{}", ppm);
    }

    #[test]
    fn get_ppm_ends_with_nl() {
        let mut canvas = Canvas::new(5, 3);
        canvas.clear(Some(Color::rgb(1.0, 0.8, 0.6)));
        let ppm = canvas.into_ppm_string();
        assert_eq!(ppm.chars().last().unwrap(), '\n');
    }
}
