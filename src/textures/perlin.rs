use crate::data::point64::Point64;
use crate::data::vector3::rand_range;
use nalgebra::Vector3;
use rand::Rng;

const POINT_COUNT: usize = 256;

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
            *elt = rand_range(-1., 1.);
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
    let mut rng = rand::thread_rng();

    (1..n).rev().for_each(|i| {
        let target = rng.gen_range(0..i);
        arr.swap(i, target);
    });
}

fn trilinear_interp(c: &mut [[[Vector3<f64>; 2]; 2]; 2], uvw: (f64, f64, f64)) -> f64 {
    let (u, v, w) = (
        uvw.0.powi(2) * (3. - 2. * uvw.0),
        uvw.1.powi(2) * (3. - 2. * uvw.1),
        uvw.2.powi(2) * (3. - 2. * uvw.2),
    );

    let mut accum = 0.;

    for (i, ci) in c.iter().enumerate() {
        for (j, cj) in ci.iter().enumerate() {
            for (k, ck) in cj.iter().enumerate() {
                let fi = i as f64;
                let fj = j as f64;
                let fk = k as f64;

                let weight_vec = Vector3::new(u - fi, v - fj, w - fk);

                accum += (fi * u + (1. - fi) * (1. - u))
                    * (fj * v + (1. - fj) * (1. - v))
                    * (fk * w + (1. - fk) * (1. - w))
                    * weight_vec.dot(ck);
            }
        }
    }

    accum
}
