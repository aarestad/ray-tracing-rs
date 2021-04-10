use crate::data::color64::Color64;
use crate::data::point64::Point64;
use crate::data::vec3_64::Vec3_64;
use crate::textures::perlin::PerlinGenerator;
use crate::textures::Texture;

pub struct Noise {
    pub(crate) noise_gen: PerlinGenerator,
    pub(crate) scale: f64,
}

impl Texture for Noise {
    fn value(&self, _u: f64, _v: f64, point: &Point64) -> Color64 {
        Color64(
            Vec3_64(1., 1., 1.) * 0.5 * (1. + self.noise_gen.noise(&Point64(self.scale * **point))),
        )
    }
}
