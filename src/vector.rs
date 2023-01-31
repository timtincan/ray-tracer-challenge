use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector(pub f64, pub f64, pub f64);

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn abs(&self) -> f64 {
        (self.0.powf(2.0) + self.1.powf(2.0) + self.2.powf(2.0)).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        Vector::new(
            self.0 / self.abs(),
            self.1 / self.abs(),
            self.2 / self.abs(),
        )
    }

    pub fn dot(&self, rhs: &Vector) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(&self, rhs: &Vector) -> Vector {
        Vector::new(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector::new(-self.0, -self.1, -self.2)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector::new(rhs.0 * self, rhs.1 * self, rhs.2 * self)
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Vector {
        Vector::new(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

// TODO: implement PartialEq

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_creation() {
        let new_vec = Vector::new(4.0, -4.0, 3.0);

        assert_eq!(new_vec.0, 4.0);
        assert_eq!(new_vec.1, -4.0);
        assert_eq!(new_vec.2, 3.0);
    }

    #[test]
    fn sub_two_vec() {
        let new_vec = Vector::new(3.0, 2.0, 1.0);
        let new_vec2 = Vector::new(5.0, 6.0, 7.0);

        assert_eq!(new_vec - new_vec2, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn neg_vec() {
        let new_vec = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(-new_vec, Vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn scalar_mul() {
        let new_vec = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(3.5 * new_vec, Vector::new(3.5, -7.0, 10.5));
    }

    #[test]
    fn scalar_div() {
        let new_vec = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(new_vec / 2.0, Vector::new(0.5, -1.0, 1.5));
    }

    #[test]
    fn magnitude_1() {
        let new_vec = Vector::new(1.0, 0.0, 0.0);

        assert_eq!(new_vec.abs(), 1.0);
    }

    #[test]
    fn magnitude_2() {
        let new_vec = Vector::new(0.0, 1.0, 0.0);

        assert_eq!(new_vec.abs(), 1.0);
    }

    #[test]
    fn magnitude_3() {
        let new_vec = Vector::new(0.0, 0.0, 1.0);

        assert_eq!(new_vec.abs(), 1.0);
    }

    #[test]
    fn magnitude_4() {
        let new_vec = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(new_vec.abs(), 14.0_f64.sqrt());
    }

    #[test]
    fn magnitude_5() {
        let new_vec = Vector::new(-1.0, -2.0, -3.0);

        assert_eq!(new_vec.abs(), 14.0_f64.sqrt());
    }

    #[test]
    fn normalize_1() {
        let new_vec = Vector::new(4.0, 0.0, 0.0);

        assert_eq!(new_vec.normalize(), Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn normalize_2() {
        let new_vec = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(
            new_vec.normalize(),
            Vector::new(
                1.0 / 14.0_f64.sqrt(),
                2.0 / 14.0_f64.sqrt(),
                3.0 / 14.0_f64.sqrt()
            )
        )
    }

    #[test]
    fn mag_of_normalized_vec() {
        let new_vec = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(new_vec.normalize().abs(), 1.0);
    }

    #[test]
    fn dot_prod() {
        let new_vec_1 = Vector::new(1.0, 2.0, 3.0);
        let new_vec_2 = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(new_vec_1.dot(&new_vec_2), 20.0);
    }

    #[test]
    fn cross_prod() {
        let new_vec_1 = Vector::new(1.0, 2.0, 3.0);
        let new_vec_2 = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(new_vec_1.cross(&new_vec_2), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(new_vec_2.cross(&new_vec_1), Vector::new(1.0, -2.0, 1.0));
    }


}