use crate::data::color64::Color64;
use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::hittables::Hittable;
use image::Rgb;

const WHITE: Color64 = Color64::new(1.0, 1.0, 1.0);
const LIGHT_BLUE: Color64 = Color64::new(0.5, 0.7, 1.0);
const BLACK: Color64 = Color64::new(0.0, 0.0, 0.0);

pub fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color64 {
    if depth < 1 {
        return BLACK;
    }

    let hit_record = world.is_hit_by(&ray, 0.001, f64::INFINITY);

    match hit_record {
        Some(hit_record) => match hit_record.material.scatter(ray, &hit_record) {
            Some(scatter_record) => Color64(
                *(scatter_record.attenuation)
                    * *ray_color(&scatter_record.scattered, world, depth - 1),
            ),
            None => BLACK,
        },

        None => {
            let unit_direction = Point64((*ray.direction).normalized());
            let color_factor = 0.5 * (unit_direction.y() + 1.0);
            let white_amt = (1.0 - color_factor) * *WHITE;
            let blue_amt = color_factor * *LIGHT_BLUE;
            Color64(white_amt + blue_amt)
        }
    }
}

pub fn get_rgb(pixel_color: &Color64, samples_per_pixel: i32) -> Rgb<u8> {
    let mut r = pixel_color.r();
    let mut g = pixel_color.g();
    let mut b = pixel_color.b();

    let scale = 1.0 / (samples_per_pixel as f64);
    // Gamma correct for gamma = 2.0
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let scaled_red = (256.0 * r.clamp(0.0, 0.999)) as u8;
    let scaled_green = (256.0 * g.clamp(0.0, 0.999)) as u8;
    let scaled_blue = (256.0 * b.clamp(0.0, 0.999)) as u8;

    Rgb([scaled_red, scaled_green, scaled_blue])
}
