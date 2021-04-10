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

    pub fn noise(&self, p: &Point64) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = &mut [[[0.; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.random_floats[self.perm_x
                        [((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }

        trilinear_interp(&mut c, u, v, w)
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

fn trilinear_interp(c: &mut [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut accum = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let fi = i as f64;
                let fj = j as f64;
                let fk = k as f64;

                accum += (fi * u + (1. - fi) * (1. - u))
                    * (fj * v + (1. - fj) * (1. - v))
                    * (fk * w + (1. - fk) * (1. - w))
                    * c[i][j][k];
            }
        }
    }

    accum
}
