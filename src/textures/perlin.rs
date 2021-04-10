use crate::data::point64::Point64;
use rand::Rng;

const POINT_COUNT: usize = 256;

pub(crate) struct PerlinGenerator {
    pub random_floats: [f64; POINT_COUNT],
    pub perm_x: [usize; POINT_COUNT],
    pub perm_y: [usize; POINT_COUNT],
    pub perm_z: [usize; POINT_COUNT],
}

impl PerlinGenerator {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut random_floats = [0.; 256];
        rng.fill(&mut random_floats);

        Self {
            random_floats,
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub fn noise(&self, point: &Point64) -> f64 {
        let mut uvw = (
            point.x() - point.x().floor(),
            point.y() - point.y().floor(),
            point.z() - point.z().floor(),
        );

        uvw = (
            uvw.0.powi(2) * (3. - 2. * uvw.0),
            uvw.1.powi(2) * (3. - 2. * uvw.1),
            uvw.2.powi(2) * (3. - 2. * uvw.2),
        );

        let ijk = (
            point.x().floor() as i32,
            point.y().floor() as i32,
            point.z().floor() as i32,
        );

        let mut c = &mut [[[0.; 2]; 2]; 2];

        for (di, ci) in c.iter_mut().enumerate() {
            for (dj, cj) in ci.iter_mut().enumerate() {
                for (dk, ck) in cj.iter_mut().enumerate() {
                    *ck = self.random_floats[self.perm_x[((ijk.0 + di as i32) & 255) as usize]
                        ^ self.perm_y[((ijk.1 + dj as i32) & 255) as usize]
                        ^ self.perm_z[((ijk.2 + dk as i32) & 255) as usize]];
                }
            }
        }

        trilinear_interp(&mut c, uvw)
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

fn trilinear_interp(c: &mut [[[f64; 2]; 2]; 2], uvw: (f64, f64, f64)) -> f64 {
    let mut accum = 0.0;

    for (i, ci) in c.iter().enumerate() {
        for (j, cj) in ci.iter().enumerate() {
            for (k, ck) in cj.iter().enumerate() {
                let fi = i as f64;
                let fj = j as f64;
                let fk = k as f64;

                accum += (fi * uvw.0 + (1. - fi) * (1. - uvw.0))
                    * (fj * uvw.1 + (1. - fj) * (1. - uvw.1))
                    * (fk * uvw.2 + (1. - fk) * (1. - uvw.2))
                    * ck;
            }
        }
    }

    accum
}
