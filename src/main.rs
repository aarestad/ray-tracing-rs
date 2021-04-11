use std::sync::mpsc::channel;
use std::sync::Arc;

use image::{DynamicImage, ImageResult, Rgb, RgbImage};
use rand::Rng;
use threadpool::ThreadPool;

use crate::data::color64::{BLACK, LIGHT_BLUE, WHITE};
use crate::textures::noise::NoiseType::{Marble, Perlin, Turbulence};
use crate::util::worlds::{earf, random_world, simple_light, two_perlin_spheres, two_spheres};
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
mod textures;
mod util;

fn main() -> ImageResult<()> {
    let args: Vec<String> = env::args().collect();
    let options = parse_args(&args).expect("bad args!");

    // Image
    let aspect_ratio = 16. / 9.;
    let image_width: u32 = 960;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 400;
    let mut image = RgbImage::new(image_width, image_height);

    // World
    let world_choice = 6;

    let (background, world) = match world_choice {
        0 => (
            LIGHT_BLUE,
            random_world(options.create_little_spheres, options.use_bvh),
        ),
        1 => (LIGHT_BLUE, two_spheres()),
        2 => (LIGHT_BLUE, two_perlin_spheres(Perlin)),
        3 => (LIGHT_BLUE, two_perlin_spheres(Turbulence)),
        4 => (LIGHT_BLUE, two_perlin_spheres(Marble)),
        5 => (WHITE, earf()),
        6 => (BLACK, simple_light()),
        _ => panic!("bad world choice: {}", world_choice),
    };

    // Camera
    // let look_from = Point64::new(13., 2., 3.);
    // let look_at = Point64::new(0., 0., 0.);
    let look_from = Point64::new(26., 3., 6.);
    let look_at = Point64::new(0., 2., 0.);

    let camera = Arc::new(Camera::new(
        look_from,
        look_at,
        Vec3_64(0., 1., 0.),
        20.,
        aspect_ratio,
        0.,
        (*look_from - *look_at).magnitude(),
        0.,
        1.,
    ));

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel::<(u32, u32, Rgb<u8>)>();

    (0..image_height).for_each(|y| {
        (0..image_width).for_each(|x| {
            let tx = tx.clone();
            let world = world.clone();
            let camera = camera.clone();

            pool.execute(move || {
                let mut pixel_color = Color64::new(0., 0., 0.);
                let mut rng = rand::thread_rng();

                for _ in 0..samples_per_pixel {
                    let rands: [f64; 2] = rng.gen();

                    let u = (x as f64 + rands[0]) / (image_width - 1) as f64;
                    let v = (y as f64 + rands[1]) / (image_height - 1) as f64;
                    let ray = camera.get_ray(u, v);

                    *pixel_color += *ray.color_in_world(world.as_ref(), &background);
                }

                tx.send((
                    x,
                    image_height - y - 1,
                    pixel_color.to_image_rgbu8(samples_per_pixel),
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
