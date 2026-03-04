use crate::camera::Camera;
use crate::data::color64::{BLACK, Color64, LIGHT_BLUE};
use crate::data::point64::Point64;
use crate::data::vector3::{rand_range, random_in_unit_cube};
use crate::hittables::Hittables::{
    AxisAlignedRect, HittableVec, MovingSphere, Sphere, Translation,
};
use crate::hittables::{AxisAlignment, Hittables};
use crate::materials::Materials;
use crate::textures::perlin::PerlinGenerator;
use crate::textures::{NoiseType, Textures};
use nalgebra::Vector3;
use rand::RngExt;
use std::ops::Range;

pub(crate) struct World {
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub background_color: Color64,
    pub camera: Camera,
    pub hittable: Hittables,
}

const DEFAULT_LOOK_FROM: Point64 = Point64::new(13., 2., 3.);
const DEFAULT_LOOK_AT: Point64 = Point64::new(0., 0., 0.);
const DEFAULT_SAMPLES_PER_PIXEL: u32 = 200;
const DEFAULT_VUP: Vector3<f64> = Vector3::new(0., 1., 0.);
const DEFAULT_IMAGE_WIDTH: u32 = 960;
const DEFAULT_IMAGE_HEIGHT: u32 = 540;
const DEFAULT_VFOV_DEG: f64 = 40.;
const DEFAULT_APERTURE: f64 = 0.;
const DEFAULT_FOCUS_DISTANCE: f64 = 10.;
const DEFAULT_EXPOSURE_TIME: Range<f64> = 0.0..1.0;

impl World {
    pub const fn total_pixels(&self) -> u32 {
        self.image_height * self.image_width
    }

    pub fn random_world(use_bvh: bool) -> World {
        let checker_pattern = Textures::Checker(
            Box::from(Textures::SolidColor(Color64::new(0.2, 0.3, 0.1))),
            Box::from(Textures::SolidColor(Color64::new(0.9, 0.9, 0.9))),
        );

        let mut hittables: Vec<Hittables> = vec![Sphere(
            Point64::new(0., -1000., 0.),
            1000.,
            Materials::Lambertian(checker_pattern),
        )];

        let glass = Materials::Dielectric(1.5);

        let mut rng = rand::rng();

        let reference_point = Point64::new(4., 0.2, 0.);

        for a in 0..22 {
            for b in 0..22 {
                let choose_mat = rng.random::<f64>();
                let center = Point64::new(
                    (a - 11) as f64 + 0.9 * rng.random::<f64>(),
                    0.2,
                    (b - 11) as f64 + 0.9 * rng.random::<f64>(),
                );

                if (center - reference_point).0.magnitude() > 0.9 {
                    if choose_mat < 0.1 {
                        // 10% moving Lambertian spheres
                        hittables.push(Hittables::MovingSphere(
                            center,
                            Point64(center.0 + Vector3::new(0., rng.random(), 0.)),
                            0.2,
                            Materials::Lambertian(Textures::SolidColor(Color64(
                                random_in_unit_cube().component_mul(&random_in_unit_cube()),
                            ))),
                            0.,
                            1.,
                        ));
                    } else if choose_mat < 0.8 {
                        // 70% stationary Lambertian spheres
                        hittables.push(Sphere(
                            center,
                            0.2,
                            Materials::Lambertian(Textures::SolidColor(Color64(
                                random_in_unit_cube().component_mul(&random_in_unit_cube()),
                            ))),
                        ));
                    } else if choose_mat < 0.95 {
                        // 15% metal spheres
                        hittables.push(Sphere(
                            center,
                            0.2,
                            Materials::Metal(
                                Color64(rand_range(0.5, 1.)),
                                rng.random_range(0.0..0.5),
                            ),
                        ));
                    } else {
                        // 5% glass
                        hittables.push(Sphere(center, 0.2, glass.clone()));
                    }
                }
            }
        }

        hittables.push(Sphere(Point64::new(0.0, 1.0, 0.0), 1.0, glass));

        hittables.push(Sphere(
            Point64::new(-4.0, 1.0, 0.0),
            1.0,
            Materials::Lambertian(Textures::SolidColor(Color64::new(0.4, 0.2, 0.1))),
        ));

        hittables.push(Sphere(
            Point64::new(4.0, 1.0, 0.0),
            1.0,
            Materials::Metal(Color64::new(0.7, 0.6, 0.5), 0.),
        ));

        let hittable = if use_bvh {
            Hittables::new_bvh(hittables, 0.0, 1.0)
        } else {
            HittableVec(hittables.into_iter().map(|h| Box::from(h)).collect())
        };

        World {
            image_width: DEFAULT_IMAGE_WIDTH,
            image_height: DEFAULT_IMAGE_HEIGHT,
            samples_per_pixel: DEFAULT_SAMPLES_PER_PIXEL,
            background_color: LIGHT_BLUE,
            camera: Camera::new(
                DEFAULT_LOOK_FROM,
                DEFAULT_LOOK_AT,
                DEFAULT_VUP,
                20.0,
                DEFAULT_IMAGE_WIDTH as f64 / DEFAULT_IMAGE_HEIGHT as f64,
                0.1,
                DEFAULT_FOCUS_DISTANCE,
                DEFAULT_EXPOSURE_TIME,
            ),
            hittable,
        }
    }

    pub fn two_spheres() -> World {
        let material = Materials::Lambertian(Textures::Checker(
            Box::from(Textures::SolidColor(Color64::new(0.2, 0.3, 0.1))),
            Box::from(Textures::SolidColor(Color64::new(0.9, 0.9, 0.9))),
        ));

        let hittable = Hittables::HittableVec(vec![
            Sphere(Point64::new(0., -10., 0.), 10., material.clone()).into(),
            Sphere(Point64::new(0., 10., 0.), 10., material).into(),
        ]);

        World {
            image_width: DEFAULT_IMAGE_WIDTH,
            image_height: DEFAULT_IMAGE_HEIGHT,
            samples_per_pixel: DEFAULT_SAMPLES_PER_PIXEL,
            background_color: LIGHT_BLUE,
            camera: Camera::new(
                DEFAULT_LOOK_FROM,
                DEFAULT_LOOK_AT,
                DEFAULT_VUP,
                20.0,
                DEFAULT_IMAGE_WIDTH as f64 / DEFAULT_IMAGE_HEIGHT as f64,
                0.0,
                DEFAULT_FOCUS_DISTANCE,
                DEFAULT_EXPOSURE_TIME,
            ),
            hittable,
        }
    }

    pub fn two_perlin_spheres(noise_type: NoiseType) -> World {
        let material =
            Materials::Lambertian(Textures::Noise(PerlinGenerator::new(), 4., noise_type));

        let hittable = HittableVec(vec![
            Sphere(Point64::new(0., -1000., 0.), 1000., material.clone()).into(),
            Sphere(Point64::new(0., 2., 0.), 2., material).into(),
        ]);

        World {
            image_width: DEFAULT_IMAGE_WIDTH,
            image_height: DEFAULT_IMAGE_HEIGHT,
            samples_per_pixel: DEFAULT_SAMPLES_PER_PIXEL,
            background_color: LIGHT_BLUE,
            camera: Camera::new(
                DEFAULT_LOOK_FROM,
                DEFAULT_LOOK_AT,
                DEFAULT_VUP,
                20.0,
                DEFAULT_IMAGE_WIDTH as f64 / DEFAULT_IMAGE_HEIGHT as f64,
                DEFAULT_APERTURE,
                DEFAULT_FOCUS_DISTANCE,
                DEFAULT_EXPOSURE_TIME,
            ),
            hittable,
        }
    }

    pub fn earth() -> World {
        let hittable = Sphere(
            Point64::new(0., 0., 0.),
            2.,
            Materials::Lambertian(Textures::new_image("resources/earthmap.jpg".into())),
        );

        World {
            image_width: DEFAULT_IMAGE_WIDTH,
            image_height: DEFAULT_IMAGE_HEIGHT,
            samples_per_pixel: DEFAULT_SAMPLES_PER_PIXEL,
            background_color: LIGHT_BLUE,
            camera: Camera::new(
                DEFAULT_LOOK_FROM,
                DEFAULT_LOOK_AT,
                DEFAULT_VUP,
                20.0,
                DEFAULT_IMAGE_WIDTH as f64 / DEFAULT_IMAGE_HEIGHT as f64,
                DEFAULT_APERTURE,
                DEFAULT_FOCUS_DISTANCE,
                DEFAULT_EXPOSURE_TIME,
            ),
            hittable,
        }
    }

    pub fn simple_light() -> World {
        let material = Materials::Lambertian(Textures::Noise(
            PerlinGenerator::new(),
            4.,
            NoiseType::Marble,
        ));

        let light = Materials::DiffuseLight(Textures::SolidColor(Color64::new(4., 4., 4.)));

        let hittable = HittableVec(vec![
            Sphere(Point64::new(0., -1000., 0.), 1000., material.clone()).into(),
            Sphere(Point64::new(0., 2., 0.), 2., material).into(),
            AxisAlignedRect(light, (3., 1.), (5., 3.), -2., AxisAlignment::Z).into(),
        ]);

        World {
            image_width: DEFAULT_IMAGE_WIDTH,
            image_height: DEFAULT_IMAGE_HEIGHT,
            samples_per_pixel: 400,
            background_color: BLACK,
            camera: Camera::new(
                Point64::new(26., 3., 6.),
                Point64::new(0., 2., 0.),
                DEFAULT_VUP,
                20.0,
                DEFAULT_IMAGE_WIDTH as f64 / DEFAULT_IMAGE_HEIGHT as f64,
                DEFAULT_APERTURE,
                DEFAULT_FOCUS_DISTANCE,
                DEFAULT_EXPOSURE_TIME,
            ),
            hittable,
        }
    }

    pub fn cornell_box() -> World {
        let red_material =
            Materials::Lambertian(Textures::SolidColor(Color64::new(0.65, 0.05, 0.05)));

        let gray_material = Materials::Lambertian(Textures::SolidColor(Color64::gray(0.73)));

        let green_material =
            Materials::Lambertian(Textures::SolidColor(Color64::new(0.12, 0.45, 0.15)));

        let light_source = Materials::DiffuseLight(Textures::SolidColor(Color64::gray(15.)));

        let hittable = HittableVec(vec![
            AxisAlignedRect(
                green_material,
                (0., 0.),
                (555., 555.),
                555.,
                AxisAlignment::X,
            )
            .into(),
            AxisAlignedRect(red_material, (0., 0.), (555., 555.), 0., AxisAlignment::X).into(),
            AxisAlignedRect(
                light_source,
                (213., 227.),
                (343., 332.),
                554.,
                AxisAlignment::Y,
            )
            .into(),
            AxisAlignedRect(
                gray_material.clone(),
                (0., 0.),
                (555., 555.),
                0.,
                AxisAlignment::Y,
            )
            .into(),
            AxisAlignedRect(
                gray_material.clone(),
                (0., 0.),
                (555., 555.),
                555.,
                AxisAlignment::Y,
            )
            .into(),
            AxisAlignedRect(
                gray_material.clone(),
                (0., 0.),
                (555., 555.),
                555.,
                AxisAlignment::Z,
            )
            .into(),
            Translation(
                Hittables::new_cuboid(
                    Point64::new(0., 0., 0.),
                    Point64::new(165., 330., 165.),
                    gray_material.clone(),
                )
                .into(),
                Vector3::new(265., 0., 295.),
            )
            .into(),
            Translation(
                Hittables::new_cuboid(
                    Point64::new(0., 0., 0.),
                    Point64::new(165., 165., 165.),
                    gray_material,
                )
                .into(),
                Vector3::new(130., 0., 65.),
            )
            .into(),
        ]);

        World {
            image_width: 600,
            image_height: 600,
            samples_per_pixel: DEFAULT_SAMPLES_PER_PIXEL,
            background_color: BLACK,
            camera: Camera::new(
                Point64::new(278., 278., -800.),
                Point64::new(278., 278., 0.),
                DEFAULT_VUP,
                DEFAULT_VFOV_DEG,
                1.,
                DEFAULT_APERTURE,
                DEFAULT_FOCUS_DISTANCE,
                DEFAULT_EXPOSURE_TIME,
            ),
            hittable,
        }
    }

    pub fn final_scene() -> World {
        let mut rng = rand::rng();

        let ground = Materials::Lambertian(Textures::SolidColor(Color64::new(0.48, 0.83, 0.53)));

        // create the floor boxes
        let mut boxes: Vec<Hittables> = vec![];

        let boxes_per_side = 20;

        (0..boxes_per_side).for_each(|i| {
            (0..boxes_per_side).for_each(|j| {
                let w = 100.0;
                let x0 = -1000.0 + i as f64 * w;
                let z0 = -1000.0 + j as f64 * w;
                let y0 = 0.0;
                let x1 = x0 + w;
                let y1 = rng.random_range(1.0..101.0);
                let z1 = z0 + w;

                boxes.push(Hittables::new_cuboid(
                    Point64::new(x0, y0, z0),
                    Point64::new(x1, y1, z1),
                    ground.clone(),
                ));
            });
        });

        // create the box of spheres
        let mut box_of_spheres: Vec<Hittables> = vec![];

        let white_ish = Materials::Lambertian(Textures::SolidColor(Color64::gray(0.73)));

        let num_of_spheres_in_box = 1000;

        (0..num_of_spheres_in_box).for_each(|_| {
            box_of_spheres.push(Sphere(
                Point64(rand_range(0., 165.)),
                10.,
                white_ish.clone(),
            ));
        });

        World {
            image_width: 800,
            image_height: 800,
            samples_per_pixel: 100,
            background_color: BLACK,
            camera: Camera::new(
                Point64::new(478., 278., -600.),
                Point64::new(278., 278., 0.),
                DEFAULT_VUP,
                DEFAULT_VFOV_DEG,
                1.,
                DEFAULT_APERTURE,
                DEFAULT_FOCUS_DISTANCE,
                DEFAULT_EXPOSURE_TIME,
            ),
            hittable: (HittableVec(vec![
                // floor
                Hittables::new_bvh(boxes, 0., 1.).into(),
                // light
                AxisAlignedRect(
                    Materials::DiffuseLight(Textures::SolidColor(Color64::new(7., 7., 7.))),
                    (123.0, 147.0),
                    (423.0, 412.0),
                    554.0,
                    AxisAlignment::Y,
                )
                .into(),
                // // moving sphere
                MovingSphere(
                    Point64::new(400., 400., 200.),
                    Point64::new(430., 400., 200.),
                    50.0,
                    Materials::Lambertian(Textures::SolidColor(Color64::new(0.7, 0.3, 0.1))),
                    0.0,
                    1.0,
                )
                .into(),
                // // dielectric sphere
                Sphere(
                    Point64::new(260., 150., 45.),
                    50.0,
                    Materials::Dielectric(1.5),
                )
                .into(),
                // metal sphere
                Sphere(
                    Point64::new(0., 150., 145.),
                    50.0,
                    Materials::Metal(Color64::new(0.8, 0.8, 0.9), 1.0),
                )
                .into(),
                // TODO blue subsurface reflection sphere
                //
                // earth
                Sphere(
                    Point64::new(400., 200., 400.),
                    100.,
                    Materials::Lambertian(Textures::new_image(
                        "resources/earthmap.jpg".to_string(),
                    )),
                )
                .into(),
                // Perlin noise sphere
                Sphere(
                    Point64::new(220., 280., 300.),
                    80.,
                    Materials::Lambertian(Textures::Noise(
                        PerlinGenerator::new(),
                        0.1,
                        NoiseType::Perlin,
                    )),
                )
                .into(),
                // rotated/translated box of spheres
                // TODO rotation
                Translation(
                    Hittables::new_bvh(box_of_spheres, 0.0, 1.0).into(),
                    Vector3::new(-100., 270., 395.),
                )
                .into(),
            ])),
        }
    }
}
