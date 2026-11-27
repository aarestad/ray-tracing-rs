use crate::data::point64::Point64;
use crate::data::vector3::rand_range;
use nalgebra::Vector3;
use rand::Rng;

const POINT_COUNT: usize = 256;

#[derive(Clone)]
pub(crate) struct PerlinGenerator {
    pub random_vecs: [Vector3<f64>; POINT_COUNT],
    pub perm_x: [usize; POINT_COUNT],
    pub perm_y: [usize; POINT_COUNT],
    pub perm_z: [usize; POINT_COUNT],
}

impl PerlinGenerator {
    pub fn new() -> Self {
        let mut random_vecs = [Vector3::new(0., 0., 0.); 256];

        for elt in random_vecs.iter_mut() {
            // Book 2, section 5.5: random *unit* gradient vectors.
            *elt = rand_range(-1., 1.).normalize();
        }

        Self {
            random_vecs,
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub fn noise(&self, point: &Point64) -> f64 {
        let uvw = (
            point.x() - point.x().floor(),
            point.y() - point.y().floor(),
            point.z() - point.z().floor(),
        );

        let ijk = (
            point.x().floor() as i32,
            point.y().floor() as i32,
            point.z().floor() as i32,
        );

        let c = &mut [[[Vector3::new(0., 0., 0.); 2]; 2]; 2];

        for (di, ci) in c.iter_mut().enumerate() {
            for (dj, cj) in ci.iter_mut().enumerate() {
                for (dk, ck) in cj.iter_mut().enumerate() {
                    *ck = self.random_vecs[self.perm_x[((ijk.0 + di as i32) & 255) as usize]
                        ^ self.perm_y[((ijk.1 + dj as i32) & 255) as usize]
                        ^ self.perm_z[((ijk.2 + dk as i32) & 255) as usize]];
                }
            }
        }

        trilinear_interp(c, uvw)
    }

    pub fn turbulence(&self, point: &Point64, depth: i32) -> f64 {
        let mut accum = 0.;
        let mut temp_p = *point;
        let mut weight = 1.;

        (0..depth).for_each(|_| {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.;
        });

        accum.abs()
    }
}

fn perlin_generate_perm() -> [usize; POINT_COUNT] {
    let mut p = array_init::array_init(|i| i);
    permute(&mut p, POINT_COUNT);
    p
}

fn permute(arr: &mut [usize; POINT_COUNT], n: usize) {
    let mut rng = rand::rng();

    (1..n).rev().for_each(|i| {
        // Fisher–Yates: `target` may equal `i` (an exclusive range would be Sattolo's
        // algorithm, which only produces single-cycle permutations).
        let target = rng.random_range(0..=i);
        arr.swap(i, target);
    });
}

fn trilinear_interp(c: &mut [[[Vector3<f64>; 2]; 2]; 2], uvw: (f64, f64, f64)) -> f64 {
    // Hermitian-smoothed blend weights (book 2, section 5.3). The gradient dot products below
    // use the *raw* fractional offsets (u, v, w); only the blend uses the smoothed ones.
    let (u, v, w) = uvw;
    let (uu, vv, ww) = (
        u.powi(2) * (3. - 2. * u),
        v.powi(2) * (3. - 2. * v),
        w.powi(2) * (3. - 2. * w),
    );

    let mut accum = 0.;

    for (i, ci) in c.iter().enumerate() {
        for (j, cj) in ci.iter().enumerate() {
            for (k, ck) in cj.iter().enumerate() {
                let fi = i as f64;
                let fj = j as f64;
                let fk = k as f64;

                let weight_vec = Vector3::new(u - fi, v - fj, w - fk);

                accum += (fi * uu + (1. - fi) * (1. - uu))
                    * (fj * vv + (1. - fj) * (1. - vv))
                    * (fk * ww + (1. - fk) * (1. - ww))
                    * weight_vec.dot(ck);
            }
        }
    }

    accum
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn permutation_tables_are_permutations() {
        let g = PerlinGenerator::new();
        for perm in [g.perm_x, g.perm_y, g.perm_z] {
            let mut sorted = perm;
            sorted.sort_unstable();
            assert!(sorted.iter().enumerate().all(|(i, &v)| i == v));
        }
    }

    #[test]
    fn gradient_vectors_are_unit_length() {
        let g = PerlinGenerator::new();
        for v in g.random_vecs.iter() {
            assert_abs_diff_eq!(v.magnitude(), 1.0, epsilon = 1e-12);
        }
    }

    #[test]
    fn noise_is_zero_on_lattice_points() {
        let g = PerlinGenerator::new();
        for &(x, y, z) in &[(0., 0., 0.), (3., -2., 7.), (-5., 1., 100.)] {
            assert_abs_diff_eq!(g.noise(&Point64::new(x, y, z)), 0.0, epsilon = 1e-12);
        }
    }

    #[test]
    fn interpolation_uses_raw_offsets_in_dot_products() {
        // With every gradient = +x, the book's formula reduces to u - smooth(u),
        // independent of v and w.
        let mut g = PerlinGenerator::new();
        g.random_vecs = [Vector3::new(1., 0., 0.); POINT_COUNT];
        let u: f64 = 0.25;
        let expected = u - u * u * (3. - 2. * u);
        assert_abs_diff_eq!(
            g.noise(&Point64::new(u, 0.5, 0.7)),
            expected,
            epsilon = 1e-12
        );
    }

    #[test]
    fn noise_is_bounded_and_continuous() {
        let g = PerlinGenerator::new();
        for _ in 0..10_000 {
            let p = Point64(rand_range(-50., 50.));
            let n = g.noise(&p);
            assert!(n.abs() <= 1.0, "noise out of range: {n}");
            // Small steps (including across cell boundaries) give small changes.
            let step = Point64(rand_range(-1e-6, 1e-6));
            assert!((g.noise(&(p + step)) - n).abs() < 1e-4);
        }
    }
}
