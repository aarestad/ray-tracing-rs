use std::sync::mpsc::channel;
use std::sync::Arc;

use image::{DynamicImage, ImageResult, Rgb, RgbImage};
use rand::Rng;
use threadpool::ThreadPool;

use camera::Camera;
use data::color64::Color64;
use data::point64::Point64;
use data::ray::Ray;
use data::vec3_64::Vec3_64;
use hittables::hittable_vec::HittableVec;
use hittables::moving_sphere::MovingSphere;
use hittables::sphere::Sphere;
use hittables::Hittable;
use materials::dielectric::Dielectric;
use materials::lambertian::Lambertian;
use materials::metal::Metal;

mod camera;
mod data;
mod hittables;
mod materials;

const WHITE: Color64 = Color64::new(1.0, 1.0, 1.0);
const LIGHT_BLUE: Color64 = Color64::new(0.5, 0.7, 1.0);
const BLACK: Color64 = Color64::new(0.0, 0.0, 0.0);

fn create_world() -> Arc<dyn Hittable + Send + Sync> {
    let mut hittables: Vec<Box<dyn Hittable + Send + Sync>> = vec![Box::new(Sphere {
        center: Point64::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian {
            color: Color64::new(0.5, 0.5, 0.5),
        }),
    })];

    let glass = Arc::new(Dielectric {
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
                if choose_mat < 0.1 {
                    // 10% moving Lambertian spheres
                    hittables.push(Box::new(MovingSphere {
                        center0: center,
                        center1: Point64(*center + Vec3_64(0.0, rng.gen(), 0.0)),
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        material: Arc::new(Lambertian {
                            color: Color64(Vec3_64::random() * Vec3_64::random()),
                        }),
                    }));
                } else if choose_mat < 0.8 {
                    // 70% stationary Lambertian spheres
                    hittables.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Lambertian {
                            color: Color64(Vec3_64::random() * Vec3_64::random()),
                        }),
                    }));
                } else if choose_mat < 0.95 {
                    // 15% metal spheres
                    hittables.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Metal {
                            albedo: Color64(Vec3_64::rand_range(0.5, 1.0)),
                            fuzz: rng.gen_range(0.0..0.5),
                        }),
                    }));
                } else {
                    // 5% glass
                    hittables.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: glass.clone(),
                    }));
                }
            }
        }
    }

    hittables.push(Box::new(Sphere {
        center: Point64::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: glass,
    }));

    hittables.push(Box::new(Sphere {
        center: Point64::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Lambertian {
            color: Color64::new(0.4, 0.2, 0.1),
        }),
    }));

    hittables.push(Box::new(Sphere {
        center: Point64::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Metal {
            albedo: Color64::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    }));

    Arc::new(HittableVec { hittables })
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
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 960;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let mut image = RgbImage::new(image_width, image_height);
    let max_depth = 50;

    // World
    let world = create_world();

    // Camera
    let look_from = Point64::new(13.0, 2.0, 3.0);
    let look_at = Point64::new(0.0, 0.0, 0.0);
    let vup = Vec3_64(0.0, 1.0, 0.0);
    let vfov_deg = 20.0;
    let focus_dist = (*look_from - *look_at).magnitude();
    let aperture = 0.1;
    let exposure_time0 = 0.0;
    let exposure_time1 = 1.0;

    let camera = Arc::new(Camera::new(
        look_from,
        look_at,
        vup,
        vfov_deg,
        aspect_ratio,
        aperture,
        focus_dist,
        exposure_time0,
        exposure_time1,
    ));

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel::<(u32, u32, Rgb<u8>)>();

    (0..image_height).for_each(|y| {
        (0..image_width).for_each(|x| {
            let tx = tx.clone();
            let world = world.clone();
            let camera = camera.clone();

            pool.execute(move || {
                let mut pixel_color = Color64::new(0.0, 0.0, 0.0);
                let mut rng = rand::thread_rng();

                for _ in 0..samples_per_pixel {
                    let rands: [f64; 2] = rng.gen();

                    let u = (x as f64 + rands[0]) / (image_width - 1) as f64;
                    let v = (y as f64 + rands[1]) / (image_height - 1) as f64;
                    let ray = camera.get_ray(u, v);

                    *pixel_color += *ray_color(&ray, world.as_ref(), max_depth);
                }

                tx.send((
                    x,
                    image_height - y - 1,
                    get_rgb(&pixel_color, samples_per_pixel),
                ))
                .expect("no receiver");
            });
        });
    });

    let mut pixel_count = 0;
    let total_pixels = image_height * image_width;

    for pixel in rx.iter() {
        image.put_pixel(pixel.0, pixel.1, pixel.2);
        pixel_count += 1;

        if pixel_count % image_width == 0 {
            println!("{} / {} scanlines done", pixel_count / image_width, image_height);
        }

        if pixel_count == total_pixels {
            break;
        }
    }

    DynamicImage::ImageRgb8(image).save("output.png")?;

    println!("Done!");

    Ok(())
}
