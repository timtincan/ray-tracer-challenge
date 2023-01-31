use crate::vector::Vector;
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Clone)]
pub struct Point(pub f64, pub f64, pub f64);

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Point {
        Point::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Vector {
        Vector::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Point {
        Point::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

// TODO: implement PartialEq

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_creation() {
        let new_point = Point::new(4.0, -4.0, 3.0);

        assert_eq!(new_point.0, 4.0);
        assert_eq!(new_point.1, -4.0);
        assert_eq!(new_point.2, 3.0);
    }

    #[test]
    fn add_point_vec() {
        let new_point = Point::new(3.0, -2.0, 5.0);
        let new_vec = Vector::new(-2.0, 3.0, 1.0);

        assert_eq!(new_point + new_vec, Point::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn sub_two_points() {
        let new_point = Point::new(3.0, 2.0, 1.0);
        let new_point2 = Point::new(5.0, 6.0, 7.0);

        assert_eq!(new_point - new_point2, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn sub_point_vec() {
        let new_point = Point::new(3.0, 2.0, 1.0);
        let new_vec = Vector::new(5.0, 6.0, 7.0);

        assert_eq!(new_point - new_vec, Point::new(-2.0, -4.0, -6.0));
    }
}
