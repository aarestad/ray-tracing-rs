use nalgebra::Vector3;
use std::ops::{Add, Div, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Point64(pub Vector3<f64>);

impl Point64 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Point64(Vector3::new(x, y, z))
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }

    pub fn z(&self) -> f64 {
        self.0[2]
    }
}

impl Add for Point64 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point64(self.0 + rhs.0)
    }
}

impl Neg for Point64 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Point64(-self.0)
    }
}

impl Sub for Point64 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point64(self.0 - rhs.0)
    }
}

impl Sub<Vector3<f64>> for Point64 {
    type Output = Self;

    fn sub(self, rhs: Vector3<f64>) -> Self::Output {
        Point64(self.0 - rhs)
    }
}

impl Mul<f64> for Point64 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Point64(self.0 * rhs)
    }
}

impl MulAssign<f64> for Point64 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs
    }
}

impl Div<f64> for Point64 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Point64(self.0 / rhs)
    }
}
