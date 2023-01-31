use crate::color::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![
                Color {
                    red: 0.0,
                    green: 0.0,
                    blue: 0.0
                };
                width * height
            ],
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[x + y * self.width]
    }
    
    pub fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.pixels[x + y * self.width] = c;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_creation() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for x in 0..10 {
            for y in 0..20 {
                assert_eq!(
                    c.pixel_at(x, y),
                    Color {
                        red: 0.0,
                        green: 0.0,
                        blue: 0.0
                    }
                );
            }
        }
    }

    #[test]
    fn write_pixel() {
        let mut c = Canvas::new(10, 20);
        let red = Color { red: 0.0, green: 0.0, blue: 0.0 };

        c.write_pixel(2, 3, red);

        assert_eq!(c.pixel_at(2, 3), red);
    }
}
