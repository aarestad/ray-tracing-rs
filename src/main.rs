#![allow(clippy::redundant_clone)] // clippy is flagging the glass.clone() on line 152

use crate::hittable_vec::HittableVec;
use camera::Camera;
use color64::Color64;
use dielectric::Dielectric;
use hittable::Hittable;
use image::{DynamicImage, ImageResult, Rgb, RgbImage};
use lambertian::Lambertian;
use material::Material;
use metal::Metal;
use point64::Point64;
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use std::rc::Rc;
use vec3_64::Vec3_64;

mod camera;
mod color64;
mod dielectric;
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

fn create_world() -> impl Hittable {
    let mut hittables: Vec<Box<dyn Hittable>> = vec![];

    hittables.push(Box::new(Sphere {
        center: Point64::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Rc::new(Lambertian {
            color: Color64::new(0.5, 0.5, 0.5),
        }),
    }));

    let glass = Rc::new(Dielectric {
        index_of_refraction: 1.5,
    });

    let mut rng = rand::thread_rng();

    let reference_point = Point64::new(4.0, 0.2, 0.0);

    for a in 0..22 {
        for b in 0..22 {
            let choose_mat = rng.gen::<f64>();
            let center = Point64::new(
                (a - 11) as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                (b - 11) as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (*center - *reference_point).magnitude() > 0.9 {
                let sphere_material: Rc<dyn Material>;

                if choose_mat < 0.8 {
                    // 80% Lambertian spheres
                    sphere_material = Rc::new(Lambertian {
                        color: Color64(Vec3_64::random() * Vec3_64::random()),
                    });
                } else if choose_mat < 0.95 {
                    // 15% metal spheres
                    sphere_material = Rc::new(Metal {
                        albedo: Color64(Vec3_64::rand_range(0.5, 1.0)),
                        fuzz: rng.gen_range(0.0..0.5),
                    });
                } else {
                    // 5% glass
                    sphere_material = glass.clone();
                }

                hittables.push(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material: sphere_material,
                }));
            }
        }
    }

    hittables.push(Box::new(Sphere {
        center: Point64::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: glass.clone(),
    }));

    hittables.push(Box::new(Sphere {
        center: Point64::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Rc::new(Lambertian {
            color: Color64::new(0.4, 0.2, 0.1),
        }),
    }));

    hittables.push(Box::new(Sphere {
        center: Point64::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Rc::new(Metal {
            albedo: Color64::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    }));

    HittableVec { hittables }
}

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color64 {
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
    let aspect_ratio = 3.0 / 2.0;
    let image_width: u32 = 1200;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 5;
    let mut image = RgbImage::new(image_width, image_height);
    let max_depth = 50;

    // World
    let world = create_world();

    // Camera
    let lookfrom = Point64::new(13.0, 2.0, 3.0);
    let lookat = Point64::new(0.0, 0.0, 0.0);
    let vup = Vec3_64(0.0, 1.0, 0.0);
    let vfov_deg = 20.0;
    let focus_dist = (*lookfrom - *lookat).magnitude();
    let aperture = 0.1;

    let camera: Camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov_deg,
        aspect_ratio,
        aperture,
        focus_dist,
    );

    let mut rng = rand::thread_rng();

    let mut pixels: Vec<(u32, u32, Rgb<u8>)> = vec![];

    for y in 0..image_height {
        eprintln!("\rScanlines remaining: {}", image_height - y);

        for x in 0..image_width {
            let p = {
                let mut pixel_color = Color64::new(0.0, 0.0, 0.0);

                for _ in 0..samples_per_pixel {
                    let rands: [f64; 2] = rng.gen();

                    let u = (x as f64 + rands[0]) / (image_width - 1) as f64;
                    let v = (y as f64 + rands[1]) / (image_height - 1) as f64;
                    let ray = camera.get_ray(u, v);

                    *pixel_color += *ray_color(&ray, &world, max_depth);
                }

                (x, image_height - y - 1, get_rgb(&pixel_color, samples_per_pixel))
            };

            pixels.push(p);
        }
    }

    for p in pixels {
        image.put_pixel(p.0, p.1, p.2);
    }


    DynamicImage::ImageRgb8(image).save("output.png")?;

    eprintln!("Done!");

    Ok(())
}
