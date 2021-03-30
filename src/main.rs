use crate::lambertian::Lambertian;
use camera::Camera;
use color64::Color64;
use hittable::Hittable;
use hittable_vec::HittableVec;
use image::{DynamicImage, ImageResult, Rgb, RgbImage};
use metal::Metal;
use point64::Point64;
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use std::rc::Rc;

mod camera;
mod color64;
mod hittable;
mod hittable_vec;
mod lambertian;
mod material;
mod metal;
mod point64;
mod ray;
mod sphere;
mod vec3_64;

const WHITE: Color64 = Color64::new(1.0, 1.0, 1.0);
const LIGHT_BLUE: Color64 = Color64::new(0.5, 0.7, 1.0);
const BLACK: Color64 = Color64::new(0.0, 0.0, 0.0);

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color64 {
    if depth < 1 {
        return BLACK;
    }

    let hit_record = world.is_hit_by(&ray, 0.001, f64::INFINITY);

    match hit_record {
        Some(hit_record) => {
            match hit_record.material.scatter(ray, &hit_record) {
                Some(scatter_record) => Color64(
                    *(scatter_record.attenuation)
                        * *ray_color(&scatter_record.scattered, world, depth - 1),
                ),
                None => BLACK,
            };

            BLACK
        }

        None => {
            let unit_direction = Point64((*ray.direction).normalized());
            let color_factor = 0.5 * (unit_direction.y() + 1.0);
            let white_amt = (1.0 - color_factor) * *WHITE;
            let blue_amt = color_factor * *LIGHT_BLUE;
            Color64(white_amt + blue_amt)
        }
    }
}

fn get_rgb(pixel_color: &Color64, samples_per_pixel: i32) -> Rgb<u8> {
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

fn main() -> ImageResult<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 800;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let mut image = RgbImage::new(image_width, image_height);
    let max_depth = 50;

    // World

    let ground = Sphere {
        center: Point64::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Rc::new(Lambertian {
            color: Color64::new(0.8, 0.8, 0.0),
        }),
    };

    let center = Sphere {
        center: Point64::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(Lambertian {
            color: Color64::new(0.7, 0.3, 0.3),
        }),
    };

    let left = Sphere {
        center: Point64::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(Metal {
            albedo: Color64::new(0.8, 0.8, 0.8),
        }),
    };

    let right = Sphere {
        center: Point64::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(Metal {
            albedo: Color64::new(0.8, 0.6, 0.2),
        }),
    };

    let world = HittableVec {
        hittables: vec![&ground, &center, &left, &right],
    };

    let camera: Camera = Camera::new();

    let mut rng = rand::thread_rng();

    for y in 0..image_height {
        eprintln!("\rScanlines remaining: {}", image_height - y);

        for x in 0..image_width {
            let mut pixel_color = Color64::new(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel {
                let rands: [f64; 2] = rng.gen();

                let u = (x as f64 + rands[0]) / (image_width - 1) as f64;
                let v = (y as f64 + rands[1]) / (image_height - 1) as f64;
                let direction = camera.direction(u, v);

                let ray = Ray {
                    origin: camera.origin,
                    direction,
                };

                *pixel_color += *ray_color(&ray, &world, max_depth);
            }

            image.put_pixel(
                x,
                image_height - y - 1,
                get_rgb(&pixel_color, samples_per_pixel),
            );
        }
    }

    DynamicImage::ImageRgb8(image).save("output.png")?;

    eprintln!("Done!");

    Ok(())
}
