use crate::data::color64::Color64;
use crate::data::point64::Point64;
use crate::textures::Texture;
use image::{DynamicImage, GenericImageView, ImageReader};
use nalgebra::Vector3;
use std::path::Path;

pub struct ImageTexture {
    image: Option<DynamicImage>,
}

impl ImageTexture {
    pub fn new(filename: String) -> Self {
        let image_file = ImageReader::open(Path::new(&filename));

        match image_file {
            Ok(image_file) => {
                let img_decoded = image_file.decode();

                match img_decoded {
                    Ok(image) => ImageTexture { image: Some(image) },
                    Err(e) => {
                        eprintln!("cold not decode image: {}", e);
                        ImageTexture { image: None }
                    }
                }
            }
            Err(e) => {
                eprintln!("cold not open file {}: {}", filename, e);
                ImageTexture { image: None }
            }
        }
    }
}

const COLOR_SCALE: f64 = 1. / 255.;

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _point: &Point64) -> Color64 {
        // If we have no texture data, then return solid cyan as a debugging aid.
        if self.image.is_none() {
            Color64::new(0., 1., 1.)
        } else {
            let image = self.image.as_ref().unwrap();

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
}
