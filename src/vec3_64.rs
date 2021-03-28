use std::ops::{Add, Mul, Neg, Deref};

#[derive(Debug, Copy, Clone)]
pub struct Vec3_64 (pub(crate) f64, pub(crate) f64, pub(crate) f64);

impl Neg for Vec3_64 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Add for Vec3_64 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Mul<f64> for Vec3_64 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vec3_64> for f64 {
    type Output = Vec3_64;

    fn mul(self, rhs: Vec3_64) -> Self::Output {
        Vec3_64(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}
