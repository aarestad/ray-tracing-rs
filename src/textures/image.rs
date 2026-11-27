use crate::data::color64::Color64;
use crate::data::point64::Point64;
use image::{ImageReader, RgbImage};
use std::path::Path;
use std::sync::Arc;

/// Gamma used to convert 8-bit image data to linear light, matching `stbi_loadf`'s default
/// LDR-to-HDR conversion that the book relies on (book 2, section 4.5).
const IMAGE_GAMMA: f64 = 2.2;

#[derive(Clone)]
pub struct ImageTexture {
    /// Linear-light pixels in row-major order, top row first. `None` if loading failed.
    pixels: Option<Arc<LinearImage>>,
}

struct LinearImage {
    width: u32,
    height: u32,
    data: Vec<Color64>,
}

impl LinearImage {
    fn from_rgb8(img: &RgbImage) -> Self {
        // 256-entry lookup table: every channel value maps to the same linear value.
        let lut: Vec<f64> = (0..=255u8)
            .map(|c| (c as f64 / 255.).powf(IMAGE_GAMMA))
            .collect();
        let data = img
            .pixels()
            .map(|p| Color64::new(lut[p[0] as usize], lut[p[1] as usize], lut[p[2] as usize]))
            .collect();
        LinearImage {
            width: img.width(),
            height: img.height(),
            data,
        }
    }
}

impl ImageTexture {
    pub fn new(filename: String) -> Self {
        let decoded = ImageReader::open(Path::new(&filename))
            .map_err(|e| format!("could not open file {filename}: {e}"))
            .and_then(|r| {
                r.decode()
                    .map_err(|e| format!("could not decode image {filename}: {e}"))
            });
        match decoded {
            Ok(image) => Self::from_rgb8(&image.to_rgb8()),
            Err(msg) => {
                eprintln!("{msg}");
                ImageTexture { pixels: None }
            }
        }
    }

    pub fn from_rgb8(img: &RgbImage) -> Self {
        ImageTexture {
            pixels: Some(Arc::new(LinearImage::from_rgb8(img))),
        }
    }

    pub fn value(&self, u: f64, v: f64, _point: &Point64) -> Color64 {
        // If we have no texture data, then return solid cyan as a debugging aid.
        let Some(img) = &self.pixels else {
            return Color64::new(0., 1., 1.);
        };

        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = u.clamp(0., 1.);
        let v = 1.0 - v.clamp(0., 1.); // Flip V to image coordinates

        let width = img.width as f64;
        let height = img.height as f64;
        let i = (u * width).min(width - 1.) as usize;
        let j = (v * height).min(height - 1.) as usize;
        img.data[j * img.width as usize + i]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_abs_diff_eq;
    use image::Rgb;

    fn two_by_two() -> ImageTexture {
        let mut img = RgbImage::new(2, 2);
        img.put_pixel(0, 0, Rgb([255, 0, 0])); // top-left
        img.put_pixel(1, 0, Rgb([0, 255, 0])); // top-right
        img.put_pixel(0, 1, Rgb([0, 0, 255])); // bottom-left
        img.put_pixel(1, 1, Rgb([128, 128, 128])); // bottom-right
        ImageTexture::from_rgb8(&img)
    }

    #[test]
    fn v_is_flipped_so_v1_is_the_top_row() {
        let t = two_by_two();
        let p = Point64::new(0., 0., 0.);
        assert_eq!(t.value(0.25, 0.75, &p), Color64::new(1., 0., 0.));
        assert_eq!(t.value(0.75, 0.75, &p), Color64::new(0., 1., 0.));
        assert_eq!(t.value(0.25, 0.25, &p), Color64::new(0., 0., 1.));
        // Out-of-range coordinates clamp to the edges.
        assert_eq!(t.value(-1., 2., &p), Color64::new(1., 0., 0.));
        assert_eq!(t.value(1., 0., &p).r(), t.value(0.75, 0.25, &p).r());
    }

    #[test]
    fn pixels_are_converted_to_linear() {
        let t = two_by_two();
        let gray = t.value(0.75, 0.25, &Point64::new(0., 0., 0.));
        assert_abs_diff_eq!(gray.r(), (128. / 255f64).powf(2.2), epsilon = 1e-12);
        assert!(gray.r() < 0.25, "mid-gray should be dark in linear light");
    }

    #[test]
    fn missing_file_is_cyan() {
        let t = ImageTexture::new("does/not/exist.png".into());
        assert_eq!(
            t.value(0.5, 0.5, &Point64::new(0., 0., 0.)),
            Color64::new(0., 1., 1.)
        );
    }
}
