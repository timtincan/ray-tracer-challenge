mod point;
mod vector;
mod color;
mod canvas;
pub use crate::point::Point;
pub use crate::vector::Vector;
pub use crate::color::Color;
pub use crate::canvas::Canvas;

pub const EPSILON: f64 = 0.0001;

pub fn float_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_equals() {
        assert!(float_eq(1.111113, 1.111115));
    }

    #[test]
    fn float_equals_2() {
        assert!(float_eq(0.000000000000000000000000000000235, 0.0));
    }

    #[test]
    fn float_equals_3() {
        assert!(float_eq(0.246899999999999, 0.2468988999999999998));
    }
}
