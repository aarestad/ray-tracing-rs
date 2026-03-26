use std::sync::Arc;
use std::sync::mpsc::channel;

use image::{ImageResult, RgbImage};
use rand::Rng;
use threadpool::ThreadPool;

use crate::textures::noise::NoiseType::{Marble, Perlin, Turbulence};
use crate::util::worlds::World;
use data::color64::Color64;
use image::DynamicImage::ImageRgb8;
use std::env;
use std::ops::AddAssign;
use util::args::parse_args;

mod camera;
mod data;
mod hittables;
mod materials;
mod textures;
mod util;

impl AddAssign for Color64 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

fn main() -> ImageResult<()> {
    let args: Vec<String> = env::args().collect();
    let options = parse_args(&args).expect("bad args!");

    if options.help {
        println!("{}", options.help_str);
        return Ok(());
    }

    let world_choice = options.world_choice;

    let mut world = match world_choice {
        0 => World::random_world(options.use_bvh),
        1 => World::two_spheres(),
        2 => World::two_perlin_spheres(Perlin),
        3 => World::two_perlin_spheres(Turbulence),
        4 => World::two_perlin_spheres(Marble),
        5 => World::earth(),
        6 => World::simple_light(),
        7 => World::cornell_box(),
        8 => World::final_scene(),
        _ => panic!("bad world choice: {}", world_choice),
    };

    world.samples_per_pixel = options.samples_per_pixel;
    let world = Arc::new(world);

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel::<(u32, Vec<Color64>)>();

    (0..world.image_height).for_each(|y| {
        let tx = tx.clone();
        let world = world.clone();

        pool.execute(move || {
            let flipped_y = world.image_height - y - 1;
            let w = world.image_width as usize;
            let mut row = Vec::with_capacity(w);

            for x in 0..world.image_width {
                let mut pixel_color = Color64::new(0., 0., 0.);
                let mut rng = rand::rng();

                for _ in 0..world.samples_per_pixel {
                    let rands: [f64; 2] = rng.random();

                    let u = (x as f64 + rands[0]) / (world.image_width - 1) as f64;
                    let v = (y as f64 + rands[1]) / (world.image_height - 1) as f64;
                    let ray = world.camera.get_ray(u, v);

                    pixel_color +=
                        ray.color_in_world(world.hittable.as_ref(), &world.background_color);
                }

                row.push(pixel_color);
            }

            tx.send((flipped_y, row)).expect("no receiver");
        });
    });

    drop(tx);

    let mut image = RgbImage::new(world.image_width, world.image_height);
    let mut rows_done = 0u32;

    for (flipped_y, row) in rx.iter() {
        for (x, pixel_color) in row.iter().enumerate() {
            image.put_pixel(
                x as u32,
                flipped_y,
                pixel_color.to_image_rgbu8(world.samples_per_pixel),
            );
        }

        rows_done += 1;
        println!(
            "{} / {} scanlines done",
            rows_done,
            world.image_height
        );

        if rows_done * world.image_width == world.total_pixels() {
            break;
        }
    }

    ImageRgb8(image).save("output.png")?;

    println!("Done!");

    Ok(())
}
