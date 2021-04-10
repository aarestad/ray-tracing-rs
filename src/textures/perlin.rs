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
        let i = (4. * p.x()) as usize & 255;
        let j = (4. * p.y()) as usize & 255;
        let k = (4. * p.z()) as usize & 255;

        self.random_floats[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
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
