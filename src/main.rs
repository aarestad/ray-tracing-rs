use std::sync::Arc;

use anyhow::Context;
use image::RgbImage;

use crate::textures::noise::NoiseType::{Marble, Perlin, Turbulence};
use crate::util::worlds::World;
use image::DynamicImage::ImageRgb8;
use std::env;
use util::args::parse_args;
use util::render::render_frame;

mod camera;
mod data;
mod hittables;
mod materials;
mod textures;
mod util;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let options = parse_args(&args)?;

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
        9 => World::utah_teapots(),
        10 => World::cornell_smoke(),
        _ => anyhow::bail!("bad world choice: {}", world_choice),
    };

    world.samples_per_pixel = options.samples_per_pixel;
    let world = Arc::new(world);

    if options.interactive {
        util::interactive::run_interactive(world)
            .context("interactive mode failed")?;
        return Ok(());
    }

    let rows = render_frame(
        world.camera.clone(),
        world.clone(),
        world.image_width,
        world.image_height,
        50,
        1,
        world.samples_per_pixel,
        None,
    )
    .expect("standalone render should not be cancelled");

    let mut image = RgbImage::new(world.image_width, world.image_height);

    for (flipped_y, row) in &rows {
        for (x, pixel_color) in row.iter().enumerate() {
            image.put_pixel(
                x as u32,
                *flipped_y,
                pixel_color.to_image_rgbu8(world.samples_per_pixel),
            );
        }
    }

    ImageRgb8(image).save("output.png")?;

    println!("Done!");

    Ok(())
}
