use std::sync::mpsc::channel;
use std::sync::Arc;

use image::{DynamicImage, ImageResult, Rgb, RgbImage};
use rand::Rng;
use threadpool::ThreadPool;

use crate::util::colors::{get_rgb, ray_color};
use crate::util::world::create_world;
use camera::Camera;
use data::color64::Color64;
use data::point64::Point64;
use data::vec3_64::Vec3_64;
use std::env;
use util::args::parse_args;

mod camera;
mod data;
mod hittables;
mod materials;
mod util;

fn main() -> ImageResult<()> {
    let args: Vec<String> = env::args().collect();
    let options = parse_args(&args).expect("bad args!");

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 960;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let mut image = RgbImage::new(image_width, image_height);
    let max_depth = 50;

    // World
    let world = create_world(options.create_little_spheres, options.use_bvh);

    // Camera
    let look_from = Point64::new(13.0, 2.0, 3.0);
    let look_at = Point64::new(0.0, 0.0, 0.0);

    let camera = Arc::new(Camera::new(
        look_from,
        look_at,
        Vec3_64(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        (*look_from - *look_at).magnitude(),
        0.0,
        1.0,
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
            println!(
                "{} / {} scanlines done",
                pixel_count / image_width,
                image_height
            );
        }

        if pixel_count == total_pixels {
            break;
        }
    }

    DynamicImage::ImageRgb8(image).save("output.png")?;

    println!("Done!");

    Ok(())
}
