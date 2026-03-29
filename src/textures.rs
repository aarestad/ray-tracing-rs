use crate::data::color64::Color64;
use crate::data::point64::Point64;

pub(crate) mod image;
pub(crate) mod noise;
pub(crate) mod perlin;
pub(crate) mod solid_color;

use crate::textures::image::ImageTexture;
use crate::textures::noise::Noise;
use crate::textures::solid_color::SolidColor;

// Larger scale -> smaller squares
const CHECKER_SCALE: f64 = 10.;

#[derive(Clone)]
pub enum Texture {
    Solid(SolidColor),
    Checker {
        odd: Box<Texture>,
        even: Box<Texture>,
    },
    Noise(Box<Noise>),
    Image(ImageTexture),
}

impl Texture {
    pub fn solid(color: Color64) -> Texture {
        Texture::Solid(SolidColor { color })
    }

    pub fn value(&self, u: f64, v: f64, point: &Point64) -> Color64 {
        match self {
            Texture::Solid(s) => s.color,
            Texture::Checker { odd, even } => {
                let sines = (CHECKER_SCALE * point.x()).sin()
                    * (CHECKER_SCALE * point.y()).sin()
                    * (CHECKER_SCALE * point.z()).sin();
                if sines < 0. {
                    odd.value(u, v, point)
                } else {
                    even.value(u, v, point)
                }
            }
            Texture::Noise(n) => n.value(u, v, point),
            Texture::Image(img) => img.value(u, v, point),
        }
    }
}
