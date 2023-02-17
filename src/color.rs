use crate::float_eq;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Color {
        Color {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        float_eq(self.red, other.red)
            && float_eq(self.green, other.green)
            && float_eq(self.blue, other.blue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_creation() {
        let c = Color {
            red: -0.5,
            green: 0.4,
            blue: 1.7,
        };

        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn add_colors() {
        let c1 = Color {
            red: 0.9,
            green: 0.6,
            blue: 0.75,
        };
        let c2 = Color {
            red: 0.7,
            green: 0.1,
            blue: 0.25,
        };

        assert_eq!(
            c1 + c2,
            Color {
                red: 1.6,
                green: 0.7,
                blue: 1.0,
            }
        );
    }

    #[test]
    fn sub_colors() {
        let c1 = Color {
            red: 0.9,
            green: 0.6,
            blue: 0.75,
        };
        let c2 = Color {
            red: 0.7,
            green: 0.1,
            blue: 0.25,
        };

        assert_eq!(
            c1 - c2,
            Color {
                red: 0.2,
                green: 0.5,
                blue: 0.5
            }
        );
    }

    #[test]
    fn scalar_mul_colors() {
        let c = Color {
            red: 0.2,
            green: 0.3,
            blue: 0.4,
        };

        assert_eq!(
            c * 2.0,
            Color {
                red: 0.4,
                green: 0.6,
                blue: 0.8
            }
        );
    }

    #[test]
    fn hadamard_prod_colors() {
        let c1 = Color {
            red: 1.0,
            green: 0.2,
            blue: 0.4,
        };
        let c2 = Color {
            red: 0.9,
            green: 1.0,
            blue: 0.1,
        };

        assert_eq!(
            c1 * c2,
            Color {
                red: 0.9,
                green: 0.2,
                blue: 0.04
            }
        );
    }
}
