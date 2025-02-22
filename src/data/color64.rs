use image::Rgb;
use nalgebra::Vector3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color64(pub Vector3<f64>);

pub const LIGHT_BLUE: Color64 = Color64::new(0.7, 0.8, 1.);
pub const BLACK: Color64 = Color64::new(0., 0., 0.);

impl Color64 {
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Color64(Vector3::new(r, g, b))
    }

    pub fn r(&self) -> f64 {
        self.0.x
    }

    pub fn g(&self) -> f64 {
        self.0.y
    }

    pub fn b(&self) -> f64 {
        self.0.z
    }

    pub fn gray(gray_level: f64) -> Self {
        Color64(Vector3::new(gray_level, gray_level, gray_level))
    }

    pub fn component_mul(self, rhs: &Color64) -> Color64 {
        Color64(self.0.component_mul(&rhs.0))
    }

    pub fn to_image_rgbu8(self, samples_per_pixel: u32) -> Rgb<u8> {
        let mut r = self.r();
        let mut g = self.g();
        let mut b = self.b();

        let scale = 1. / (samples_per_pixel as f64);

        // Gamma correct for gamma = 2.
        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        let scaled_red = (255. * r.clamp(0., 1.)) as u8;
        let scaled_green = (255. * g.clamp(0., 1.)) as u8;
        let scaled_blue = (255. * b.clamp(0., 1.)) as u8;

        Rgb([scaled_red, scaled_green, scaled_blue])
    }
}
