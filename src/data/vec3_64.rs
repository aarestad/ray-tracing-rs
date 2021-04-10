use rand::Rng;
use rand_distr::StandardNormal;
use std::f64::consts::TAU;
use std::ops::{Add, AddAssign, Div, Index, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
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

    pub fn cross(&self, rhs: &Self) -> Vec3_64 {
        Self(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn rand_range(min: f64, max: f64) -> Vec3_64 {
        let mut rng = rand::thread_rng();

        Self(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_in_unit_cube() -> Vec3_64 {
        Self::rand_range(0., 1.)
    }

    pub fn random_in_unit_sphere() -> Vec3_64 {
        let mut rng = rand::thread_rng();

        let u = rng.gen::<f64>().powf(1. / 3.);
        let x: f64 = rng.sample(StandardNormal);
        let y: f64 = rng.sample(StandardNormal);
        let z: f64 = rng.sample(StandardNormal);

        u * Vec3_64(x, y, z).normalized()
    }

    pub fn random_in_unit_disk() -> Vec3_64 {
        let mut rng = rand::thread_rng();

        let sqrt_r: f64 = rng.gen::<f64>().sqrt();
        let theta: f64 = rng.gen_range(0.0..TAU);

        Vec3_64(sqrt_r * theta.cos(), sqrt_r * theta.sin(), 0.)
    }

    pub fn random_unit_vector() -> Vec3_64 {
        Vec3_64::random_in_unit_sphere().normalized()
    }

    pub fn near_zero(&self) -> bool {
        let epsilon = 1e-8;
        self.0 < epsilon && self.1 < epsilon && self.2 < epsilon
    }

    pub fn reflect(&self, normal: &Vec3_64) -> Vec3_64 {
        *self - 2. * (*self).dot(normal) * *normal
    }

    pub fn refract(&self, normal: &Vec3_64, etai_over_etat: f64) -> Vec3_64 {
        let cos_theta = -self.dot(normal).min(1.);
        let r_out_normal = etai_over_etat * (*self + cos_theta * *normal);
        let r_out_parallel = -(1. - r_out_normal.mag_squared()).abs().sqrt() * *normal;
        r_out_normal + r_out_parallel
    }
}

impl Index<usize> for Vec3_64 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("index out of bounds: {}", index),
        }
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
