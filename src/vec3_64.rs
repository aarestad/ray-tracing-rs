use rand::rngs::ThreadRng;
use rand::Rng;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3_64(pub(crate) f64, pub(crate) f64, pub(crate) f64);

impl Vec3_64 {
    pub fn magnitude(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }

    pub fn normalized(&self) -> Self {
        *self / self.magnitude()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn random(rng: &mut ThreadRng) -> Vec3_64 {
        Self::rand_range(rng, 0.0, 1.0)
    }

    pub fn rand_range(rng: &mut ThreadRng, min: f64, max: f64) -> Vec3_64 {
        Self(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3_64 {
        let mut rand_vec: Vec3_64;

        loop {
            rand_vec = Vec3_64::rand_range(rng, -1.0, 1.0);

            if rand_vec.magnitude().powi(2) < 1.0 {
                break;
            }
        }

        rand_vec
    }
}

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

impl Add<f64> for Vec3_64 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

impl AddAssign for Vec3_64 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Sub for Vec3_64 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
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

impl Div<f64> for Vec3_64 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}
