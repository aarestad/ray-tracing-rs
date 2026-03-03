use ::image::{DynamicImage, GenericImageView, ImageReader};
use nalgebra::Vector3;
use crate::data::color64::Color64;
use crate::data::point64::Point64;
use crate::textures::perlin::PerlinGenerator;
use std::path::Path;

pub(crate) mod perlin;

#[derive(Clone)]
pub enum NoiseType {
    Perlin,
    Turbulence,
    Marble,
}

#[derive(Clone)]
pub enum Textures {
    Checker(Box<Textures> /* odd */, Box<Textures> /* even */),
    Image(Option<DynamicImage>),
    Noise(PerlinGenerator, f64 /* scale */, NoiseType),
    SolidColor(Color64),
}

const CHECKER_SCALE: f64 = 10.;
const COLOR_SCALE: f64 = 1. / 255.;

impl Textures {
    pub fn new_image(filename: String) -> Self {
        let image_file = ImageReader::open(Path::new(&filename));

        match image_file {
            Ok(image_file) => {
                let img_decoded = image_file.decode();

                match img_decoded {
                    Ok(image) => Textures::Image(Some(image)),
                    Err(e) => {
                        eprintln!("cold not decode image: {}", e);
                        Textures::Image(None)
                    }
                }
            }
            Err(e) => {
                eprintln!("cold not open file {}: {}", filename, e);
                Textures::Image(None)
            }
        }
    }
    pub fn value(&self, u: f64, v: f64, point: &Point64) -> Color64 {
        match self {
            Textures::Checker(odd, even) => {
                let sines = (CHECKER_SCALE * point.x()).sin()
                    * (CHECKER_SCALE * point.y()).sin()
                    * (CHECKER_SCALE * point.z()).sin();

                if sines < 0. {
                    odd.value(u, v, point)
                } else {
                    even.value(u, v, point)
                }
            }
            Textures::Image(image) => {
                // If we have no texture data, then return solid cyan as a debugging aid.
                if image.is_none() {
                    Color64::new(0., 1., 1.)
                } else {
                    let image = image.as_ref().unwrap();

                    // Clamp input texture coordinates to [0,1] x [1,0]
                    let u = u.clamp(0., 1.);
                    let v = 1.0 - v.clamp(0., 1.); // Flip V to image coordinates

                    let width = image.width() as f64;
                    let height = image.height() as f64;

                    let i = (u * width).min(width - 1.) as u32;
                    let j = (v * height).min(height - 1.) as u32;

                    let pixel = image.get_pixel(i, j).0;

                    Color64(Vector3::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64) * COLOR_SCALE)
                }
            }
            Textures::Noise(noise_gen, scale, noise_type) => {
                let gray_val = match noise_type {
                    NoiseType::Perlin => 0.5 * (1. + noise_gen.noise(&(point * scale))),
                    NoiseType::Turbulence => noise_gen.turbulence(&(point * scale), 7),
                    NoiseType::Marble => {
                        0.5 * (1.
                            + (scale * point.z() + 10. * noise_gen.turbulence(point, 7)).sin())
                    }
                };

                Color64::gray(gray_val)
            }
            Textures::SolidColor(color) => *color
        }
    }
}
