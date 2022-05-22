use nalgebra::Vector3;
use rand::Rng;
use rand_distr::StandardNormal;
use std::f64::consts::TAU;
use std::ops::Mul;

pub fn random_in_unit_sphere() -> Vector3<f64> {
    let mut rng = rand::thread_rng();

    let u = rng.gen::<f64>().powf(1. / 3.);
    let x: f64 = rng.sample(StandardNormal);
    let y: f64 = rng.sample(StandardNormal);
    let z: f64 = rng.sample(StandardNormal);

    u * Vector3::new(x, y, z).normalize()
}

pub fn random_unit_vector() -> Vector3<f64> {
    random_in_unit_sphere().normalize()
}

pub fn random_in_unit_disk() -> Vector3<f64> {
    let mut rng = rand::thread_rng();

    let sqrt_r: f64 = rng.gen::<f64>().sqrt();
    let theta: f64 = rng.gen_range(0.0..TAU);

    Vector3::new(sqrt_r * theta.cos(), sqrt_r * theta.sin(), 0.)
}

pub fn rand_range(min: f64, max: f64) -> Vector3<f64> {
    let mut rng = rand::thread_rng();

    Vector3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

pub fn random_in_unit_cube() -> Vector3<f64> {
    rand_range(0., 1.)
}

const EPSILON: f64 = 1e-8;

pub fn near_zero(vec: &Vector3<f64>) -> bool {
    vec[0] < EPSILON && vec[1] < EPSILON && vec[2] < EPSILON
}

pub fn reflect(vec: &Vector3<f64>, normal: &Vector3<f64>) -> Vector3<f64> {
    *vec - (normal.mul(2. * vec.dot(normal)))
}

pub fn refract(vec: &Vector3<f64>, normal: &Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    let cos_theta = -vec.dot(normal).min(1.);
    let r_out_normal = etai_over_etat * (*vec + cos_theta * *normal);
    let r_out_parallel = -(1. - r_out_normal.magnitude()).abs().sqrt() * *normal;
    r_out_normal + r_out_parallel
}
