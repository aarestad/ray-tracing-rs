use std::sync::mpsc::channel;
use std::sync::Arc;

use image::{DynamicImage, ImageResult, Rgb, RgbImage};
use rand::Rng;
use threadpool::ThreadPool;

use crate::util::worlds::{World};
use data::color64::Color64;
use std::env;
use util::args::parse_args;
use crate::textures::noise::NoiseType::{Perlin, Turbulence, Marble};

mod camera;
mod data;
mod hittables;
mod materials;
mod textures;
mod util;

fn main() -> ImageResult<()> {
    let args: Vec<String> = env::args().collect();
    let options = parse_args(&args).expect("bad args!");

    let world_choice = 7;

    let world = match world_choice {
        0 => Arc::from(World::random_world(options.create_little_spheres, options.use_bvh)),
        1 => Arc::from(World::two_spheres()),
        2 => Arc::from(World::two_perlin_spheres(Perlin)),
        3 => Arc::from(World::two_perlin_spheres(Turbulence)),
        4 => Arc::from(World::two_perlin_spheres(Marble)),
        5 => Arc::from(World::earth()),
        6 => Arc::from(World::simple_light()),
        7 => Arc::from(World::cornell_box()),
        _ => panic!("bad world choice: {}", world_choice),
    };

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel::<(u32, u32, Rgb<u8>)>();

    (0..world.image_height).for_each(|y| {
        (0..world.image_width).for_each(|x| {
            let tx = tx.clone();
            let world = world.clone();

            pool.execute(move || {
                let mut pixel_color = Color64::new(0., 0., 0.);
                let mut rng = rand::thread_rng();

                for _ in 0..world.samples_per_pixel {
                    let rands: [f64; 2] = rng.gen();

                    let u = (x as f64 + rands[0]) / (world.image_width - 1) as f64;
                    let v = (y as f64 + rands[1]) / (world.image_height - 1) as f64;
                    let ray = world.camera.get_ray(u, v);

                    *pixel_color += *ray.color_in_world(world.hittable.as_ref(), &world.background_color);
                }

                tx.send((
                    x,
                    world.image_height - y - 1,
                    pixel_color.to_image_rgbu8(world.samples_per_pixel),
                ))
                .expect("no receiver");
            });
        });
    });

    let mut pixel_count = 0;
    let total_pixels = world.total_pixels();
    let mut image = RgbImage::new(world.image_width, world.image_height);

    for pixel in rx.iter() {
        image.put_pixel(pixel.0, pixel.1, pixel.2);
        pixel_count += 1;

        if pixel_count % world.image_width == 0 {
            println!(
                "{} / {} scanlines done",
                pixel_count / world.image_width,
                world.image_height
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
