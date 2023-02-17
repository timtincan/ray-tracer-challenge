use crate::color::Color;

const MAXIMUM_PPM_LEN: usize = 70;

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

    pub fn to_ppm(&self) -> String {
        let mut buffer = ["P3", &format!("{} {}", self.width, self.height), "255"].join("\n");
        buffer.push('\n');

        let mut column_width = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.pixel_at(x, y);

                let pixel_list = [
                    format!("{}", ((pixel.red.clamp(0.0, 1.0) * 256.0) as u8)),
                    format!("{}", ((pixel.green.clamp(0.0, 1.0) * 256.0) as u8)),
                    format!("{}", ((pixel.blue.clamp(0.0, 1.0) * 256.0) as u8)),
                ];

                for color in pixel_list.iter() {
                    if column_width + color.len() + 1 > MAXIMUM_PPM_LEN {
                        buffer += "\n";
                        column_width = 0
                    }
                    if column_width != 0 {
                        buffer += " ";
                    }
                    buffer += color;
                    column_width += color.len() + 1;
                }
            }
            buffer.push('\n');
            column_width = 0;
        }
        buffer.push('\n');
        buffer
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
        let red = Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        };

        c.write_pixel(2, 3, red);

        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn to_ppm_file_1() {
        let c = Canvas::new(5, 3);
        let buffer = c.to_ppm();
        let lines = buffer.split("\n").collect::<Vec<_>>();
        assert_eq!(lines[0], "P3");
        assert_eq!(lines[1], "5 3");
        assert_eq!(lines[2], "255");
    }

    #[test]
    fn to_ppm_file_2() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color {
            red: 1.5,
            green: 0.0,
            blue: 0.0,
        };
        let c2 = Color {
            red: 0.0,
            green: 0.5,
            blue: 0.0,
        };
        let c3 = Color {
            red: -0.5,
            green: 0.0,
            blue: 1.0,
        };
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        let buffer = c.to_ppm();
        let lines = buffer.split("\n").collect::<Vec<_>>();
        assert_eq!(lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert_eq!(lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }

    #[test]
    fn ppm_not_over_70_chars() {
        let mut c = Canvas::new(10, 2);
        let color = Color {
            red: 1.0,
            green: 0.8,
            blue: 0.6,
        };
        for x in 0..10 {
            for y in 0..2 {
                c.write_pixel(x, y, color)
            }
        }
        let buffer = c.to_ppm();
        let lines = buffer.split("\n").collect::<Vec<_>>();
        assert_eq!(
            lines[3],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            lines[4],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
        assert_eq!(
            lines[5],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            lines[6],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
    }
}
