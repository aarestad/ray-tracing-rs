use nalgebra::Vector3;
use rand::Rng;
use rand_distr::Uniform;
use std::f64::consts::TAU;
use std::ops::Mul;

use crate::util::EPSILON;

pub type Vector = Vector3<f64>;

pub fn random_in_unit_sphere() -> Vector {
    let mut rng = rand::rng();

    let theta = TAU * rng.random::<f64>();
    let phi = (1.0 - 2.0 * rng.random::<f64>()).acos();
    let x = phi.sin() * theta.cos();
    let y = phi.sin() * theta.sin();
    let z = phi.cos();

    Vector3::new(x, y, z)
}

pub fn random_in_unit_disk() -> Vector {
    let mut rng = rand::rng();

    let sqrt_r: f64 = rng.random::<f64>().sqrt();
    let theta: f64 = rng.random_range(0.0..TAU);

    Vector3::new(sqrt_r * theta.cos(), sqrt_r * theta.sin(), 0.)
}

pub fn rand_range(min: f64, max: f64) -> Vector {
    let mut rng = rand::rng();

    let dist = Uniform::new_inclusive(min, max).unwrap();

    Vector3::new(rng.sample(dist), rng.sample(dist), rng.sample(dist))
}

pub fn random_in_unit_cube() -> Vector {
    rand_range(0., 1.)
}

pub fn near_zero(vec: &Vector) -> bool {
    vec[0] < EPSILON && vec[1] < EPSILON && vec[2] < EPSILON
}

pub fn reflect(vec: &Vector, normal: &Vector) -> Vector {
    let dot_prod = vec.dot(normal);
    *vec - (normal.mul(2. * dot_prod))
}

pub fn refract(vec: &Vector, normal: &Vector, etai_over_etat: f64) -> Vector {
    let cos_theta = -vec.dot(normal).min(1.);
    let r_out_normal = etai_over_etat * (*vec + cos_theta * *normal);
    let r_out_parallel = -(1. - r_out_normal.magnitude()).abs().sqrt() * *normal;
    r_out_normal + r_out_parallel
}

#[cfg(test)]
mod test {
    use super::near_zero;
    use super::rand_range;
    use super::random_in_unit_cube;
    use super::random_in_unit_disk;
    use super::random_in_unit_sphere;
    use super::reflect;
    use super::refract;
    use super::Vector;
    use super::EPSILON;
    use approx::assert_abs_diff_eq;

    #[test]
    fn accessors() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);

        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }

    #[test]
    fn unary_methods() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(-v, Vector::new(-1.0, -2.0, -3.0));
        assert_abs_diff_eq!(v.magnitude(), 3.7416573867, epsilon = EPSILON);
        assert_abs_diff_eq!(
            v.normalize(),
            Vector::new(0.2672612419, 0.53452248, 0.80178372),
            epsilon = EPSILON
        );
    }

    #[test]
    fn vector_scalar_methods() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(v * 3.0, Vector::new(3.0, 6.0, 9.0));
        assert_abs_diff_eq!(
            v / 3.0,
            Vector::new(0.33333333, 0.66666666, 1.0),
            epsilon = EPSILON
        );
    }

    #[test]
    fn vector_vector_methods() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);

        assert_eq!(v1 + v2, Vector::new(5.0, 7.0, 9.0));
        assert_eq!(v1 - v2, Vector::new(-3.0, -3.0, -3.0));
        assert_eq!(v1.component_mul(&v2), Vector::new(4.0, 10.0, 18.0));
        assert_eq!(v1.dot(&v2), 32.0);
        assert_eq!(v1.cross(&v2), Vector::new(-3.0, 6.0, -3.0));
    }

    #[test]
    fn custom_methods() {
        for _ in 0..1000 {
            assert_abs_diff_eq!(random_in_unit_sphere().magnitude(), 1.0, epsilon = EPSILON);

            let random_disk_v = random_in_unit_disk();
            assert_eq!(random_disk_v.z, 0.0);

            let rand_range_v = rand_range(-1.0, 1.0);

            assert!(-1.0 < rand_range_v.x && rand_range_v.x < 1.0);
            assert!(-1.0 < rand_range_v.y && rand_range_v.y < 1.0);
            assert!(-1.0 < rand_range_v.z && rand_range_v.z < 1.0);

            let rand_cube_v = random_in_unit_cube();

            assert!(0.0 < rand_cube_v.x && rand_cube_v.x < 1.0);
            assert!(0.0 < rand_cube_v.y && rand_cube_v.y < 1.0);
            assert!(0.0 < rand_cube_v.z && rand_cube_v.z < 1.0);
        }

        assert!(near_zero(&Vector::new(0.0, 0.0, EPSILON / 2.0)));
    }

    #[test]
    fn reflect_and_refract() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);

        assert_abs_diff_eq!(reflect(&v1, &v2), Vector::new(-255.0, -318.0, -381.0));
        assert_abs_diff_eq!(
            refract(&v1, &v2, 0.5),
            Vector::new(-6.556601564455425, -7.820751955569282, -9.084902346683137),
            epsilon = EPSILON
        );
    }
}
