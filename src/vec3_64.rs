use rand::Rng;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3_64(pub(crate) f64, pub(crate) f64, pub(crate) f64);

impl Vec3_64 {
    pub fn magnitude(&self) -> f64 {
        self.mag_squared().sqrt()
    }

    pub fn mag_squared(&self) -> f64 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    pub fn normalized(&self) -> Self {
        *self / self.magnitude()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn random() -> Vec3_64 {
        Self::rand_range(0.0, 1.0)
    }

    pub fn rand_range(min: f64, max: f64) -> Vec3_64 {
        let mut rng = rand::thread_rng();

        Self(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3_64 {
        let mut rand_vec: Vec3_64;

        loop {
            rand_vec = Vec3_64::rand_range(-1.0, 1.0);

            if rand_vec.magnitude().powi(2) < 1.0 {
                break;
            }
        }

        rand_vec
    }

    pub fn random_unit_vector() -> Vec3_64 {
        Vec3_64::random_in_unit_sphere().normalized()
    }

    pub fn near_zero(&self) -> bool {
        let epsilon = 1e-8;
        self.0 < epsilon && self.1 < epsilon && self.2 < epsilon
    }

    pub fn reflect(&self, normal: &Vec3_64) -> Vec3_64 {
        *self - 2.0 * (*self).dot(normal) * *normal
    }

    pub fn refract(&self, normal: &Vec3_64, etai_over_etat: f64) -> Vec3_64 {
        let cos_theta = -self.dot(normal).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *normal);
        let r_out_parallel = -(1.0 - r_out_perp.mag_squared()).abs().sqrt() * *normal;
        r_out_perp + r_out_parallel
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

impl Mul<Vec3_64> for Vec3_64 {
    type Output = Self;

    fn mul(self, rhs: Vec3_64) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Div<f64> for Vec3_64 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}
