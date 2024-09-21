use crate::data::color64::Color64;
use crate::data::point64::Point64;
use crate::textures::perlin::PerlinGenerator;
use crate::textures::Texture;

pub enum NoiseType {
    Perlin,
    Turbulence,
    Marble,
}

pub struct Noise {
    pub(crate) noise_gen: PerlinGenerator,
    pub(crate) scale: f64,
    pub(crate) noise_type: NoiseType,
}

impl Texture for Noise {
    fn value(&self, _u: f64, _v: f64, point: &Point64) -> Color64 {
        let gray_val = match self.noise_type {
            NoiseType::Perlin => 0.5 * (1. + self.noise_gen.noise(&(*point * self.scale))),
            NoiseType::Turbulence => self.noise_gen.turbulence(&(*point * self.scale), 7),
            NoiseType::Marble => {
                0.5 * (1.
                    + (self.scale * point.z() + 10. * self.noise_gen.turbulence(point, 7)).sin())
            }
        };

        Color64::gray(gray_val)
    }
}
