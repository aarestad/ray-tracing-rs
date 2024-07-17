use crate::camera::Camera;
use crate::data::color64::{Color64, BLACK, LIGHT_BLUE};
use crate::data::point64::Point64;
use crate::data::vector3;
use crate::data::vector3::{rand_range, random_in_unit_cube};
use crate::hittables::axis_aligned_rect::AxisAlignment::{X, Y, Z};
use crate::hittables::axis_aligned_rect::{AxisAlignedRect, AxisAlignment};
use crate::hittables::bounded_volume_hierarchy::BoundedVolumeHierarchy;
use crate::hittables::cuboid::Cuboid;
use crate::hittables::hittable_vec::HittableVec;
use crate::hittables::moving_sphere::MovingSphere;
use crate::hittables::sphere::Sphere;
use crate::hittables::translation::Translation;
use crate::hittables::Hittable;
use crate::materials::dielectric::Dielectric;
use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::textures::checker::Checker;
use crate::textures::image::ImageTexture;
use crate::textures::noise::NoiseType::Marble;
use crate::textures::noise::{Noise, NoiseType};
use crate::textures::perlin::PerlinGenerator;
use crate::textures::solid_color::SolidColor;
use nalgebra::Vector3;
use rand::Rng;
use std::ops::Range;
use std::sync::Arc;

pub(crate) struct World {
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub background_color: Color64,
    pub camera: Camera,
    pub hittable: Arc<dyn Hittable>,
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
        let checker_pattern = Checker {
            odd: SolidColor::arc_from(Color64::new(0.2, 0.3, 0.1)),
            even: SolidColor::arc_from(Color64::new(0.9, 0.9, 0.9)),
        };

        let mut hittables: Vec<Arc<dyn Hittable>> = vec![Arc::from(Sphere {
            center: Point64::new(0., -1000., 0.),
            radius: 1000.,
            material: Arc::new(Lambertian {
                albedo: Arc::new(checker_pattern),
            }),
        })];

        let glass = Arc::new(Dielectric {
            index_of_refraction: 1.5,
        });

        let mut rng = rand::thread_rng();

        let reference_point = Point64::new(4., 0.2, 0.);

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
                        hittables.push(Arc::new(MovingSphere {
                            center0: center,
                            center1: Point64(*center + Vector3::new(0., rng.gen(), 0.)),
                            time0: 0.,
                            time1: 1.,
                            radius: 0.2,
                            material: Arc::new(Lambertian {
                                albedo: SolidColor::arc_from(Color64(
                                    random_in_unit_cube().component_mul(&random_in_unit_cube()),
                                )),
                            }),
                        }));
                    } else if choose_mat < 0.8 {
                        // 70% stationary Lambertian spheres
                        hittables.push(Arc::new(Sphere {
                            center,
                            radius: 0.2,
                            material: Arc::new(Lambertian {
                                albedo: SolidColor::arc_from(Color64(
                                    random_in_unit_cube().component_mul(&random_in_unit_cube()),
                                )),
                            }),
                        }));
                    } else if choose_mat < 0.95 {
                        // 15% metal spheres
                        hittables.push(Arc::new(Sphere {
                            center,
                            radius: 0.2,
                            material: Arc::new(Metal {
                                albedo: Color64(rand_range(0.5, 1.)),
                                fuzz: rng.gen_range(0.0..0.5),
                            }),
                        }));
                    } else {
                        // 5% glass
                        hittables.push(Arc::new(Sphere {
                            center,
                            radius: 0.2,
                            material: glass.clone(),
                        }));
                    }
                }
            }
        }

        hittables.push(Arc::new(Sphere {
            center: Point64::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: glass,
        }));

        hittables.push(Arc::new(Sphere {
            center: Point64::new(-4.0, 1.0, 0.0),
            radius: 1.0,
            material: Arc::new(Lambertian {
                albedo: SolidColor::arc_from(Color64::new(0.4, 0.2, 0.1)),
            }),
        }));

        hittables.push(Arc::new(Sphere {
            center: Point64::new(4.0, 1.0, 0.0),
            radius: 1.0,
            material: Arc::new(Metal {
                albedo: Color64::new(0.7, 0.6, 0.5),
                fuzz: 0.,
            }),
        }));

        let hittable = if use_bvh {
            BoundedVolumeHierarchy::create_bvh_arc(&mut hittables, 0.0, 1.0)
        } else {
            Arc::new(HittableVec { hittables })
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
        let material = Arc::from(Lambertian {
            albedo: Arc::from(Checker {
                odd: SolidColor::arc_from(Color64::new(0.2, 0.3, 0.1)),
                even: SolidColor::arc_from(Color64::new(0.9, 0.9, 0.9)),
            }),
        });

        let hittable = Arc::new(HittableVec {
            hittables: vec![
                Arc::from(Sphere {
                    center: Point64::new(0., -10., 0.),
                    radius: 10.,
                    material: material.clone(),
                }),
                Arc::from(Sphere {
                    center: Point64::new(0., 10., 0.),
                    radius: 10.,
                    material,
                }),
            ],
        });

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
        let material = Arc::from(Lambertian {
            albedo: Arc::from(Noise {
                noise_gen: PerlinGenerator::new(),
                scale: 4.,
                noise_type,
            }),
        });

        let hittable = Arc::new(HittableVec {
            hittables: vec![
                Arc::from(Sphere {
                    center: Point64::new(0., -1000., 0.),
                    radius: 1000.,
                    material: material.clone(),
                }),
                Arc::from(Sphere {
                    center: Point64::new(0., 2., 0.),
                    radius: 2.,
                    material,
                }),
            ],
        });

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
        let hittable = Arc::from(Sphere {
            center: Point64::new(0., 0., 0.),
            radius: 2.,
            material: Arc::new(Lambertian {
                albedo: Arc::new(ImageTexture::new("resources/earthmap.jpg".into())),
            }),
        });

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
        let material = Arc::from(Lambertian {
            albedo: Arc::from(Noise {
                noise_gen: PerlinGenerator::new(),
                scale: 4.,
                noise_type: Marble,
            }),
        });

        let light = DiffuseLight::new(Color64::new(4., 4., 4.));

        let hittable = Arc::new(HittableVec {
            hittables: vec![
                Arc::from(Sphere {
                    center: Point64::new(0., -1000., 0.),
                    radius: 1000.,
                    material: material.clone(),
                }),
                Arc::from(Sphere {
                    center: Point64::new(0., 2., 0.),
                    radius: 2.,
                    material,
                }),
                Arc::from(AxisAlignedRect {
                    material: Arc::from(light),
                    min: (3., 1.),
                    max: (5., 3.),
                    axis_value: -2.,
                    axis_alignment: Z,
                }),
            ],
        });

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
        let red_material = Lambertian {
            albedo: SolidColor::arc_from(Color64::new(0.65, 0.05, 0.05)),
        };

        let gray_material = Arc::from(Lambertian {
            albedo: SolidColor::arc_from(Color64::gray(0.73)),
        });

        let green_material = Lambertian {
            albedo: SolidColor::arc_from(Color64::new(0.12, 0.45, 0.15)),
        };

        let light_source = DiffuseLight::new(Color64::gray(15.));

        let hittable = Arc::new(HittableVec {
            hittables: vec![
                Arc::from(AxisAlignedRect {
                    material: Arc::from(green_material),
                    min: (0., 0.),
                    max: (555., 555.),
                    axis_value: 555.,
                    axis_alignment: X,
                }),
                Arc::from(AxisAlignedRect {
                    material: Arc::from(red_material),
                    min: (0., 0.),
                    max: (555., 555.),
                    axis_value: 0.,
                    axis_alignment: X,
                }),
                Arc::from(AxisAlignedRect {
                    material: Arc::from(light_source),
                    min: (213., 227.),
                    max: (343., 332.),
                    axis_value: 554.,
                    axis_alignment: Y,
                }),
                Arc::from(AxisAlignedRect {
                    material: gray_material.clone(),
                    min: (0., 0.),
                    max: (555., 555.),
                    axis_value: 0.,
                    axis_alignment: Y,
                }),
                Arc::from(AxisAlignedRect {
                    material: gray_material.clone(),
                    min: (0., 0.),
                    max: (555., 555.),
                    axis_value: 555.,
                    axis_alignment: Y,
                }),
                Arc::from(AxisAlignedRect {
                    material: gray_material.clone(),
                    min: (0., 0.),
                    max: (555., 555.),
                    axis_value: 555.,
                    axis_alignment: Z,
                }),
                Arc::from(Translation {
                    hittable: Arc::new(Cuboid::new(
                        Point64::new(0., 0., 0.),
                        Point64::new(165., 330., 165.),
                        gray_material.clone(),
                    )),
                    offset: Vector3::new(265., 0., 295.),
                }),
                Arc::from(Translation {
                    hittable: Arc::new(Cuboid::new(
                        Point64::new(0., 0., 0.),
                        Point64::new(165., 165., 165.),
                        gray_material,
                    )),
                    offset: Vector3::new(130., 0., 65.),
                }),
            ],
        });

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
        let mut rng = rand::thread_rng();

        let ground = Arc::new(Lambertian {
            albedo: SolidColor::arc_from(Color64::new(0.48, 0.83, 0.53)),
        });

        // create the floor boxes
        let mut boxes: Vec<Arc<dyn Hittable>> = vec![];

        let boxes_per_side = 20;

        (0..boxes_per_side).for_each(|i| {
            (0..boxes_per_side).for_each(|j| {
                let w = 100.0;
                let x0 = -1000.0 + i as f64 * w;
                let z0 = -1000.0 + j as f64 * w;
                let y0 = 0.0;
                let x1 = x0 + w;
                let y1 = rng.gen_range(1.0..101.0);
                let z1 = z0 + w;

                boxes.push(Arc::new(Cuboid::new(
                    Point64::new(x0, y0, z0),
                    Point64::new(x1, y1, z1),
                    ground.clone(),
                )));
            });
        });

        // create the box of spheres
        let mut box_of_spheres: Vec<Arc<dyn Hittable>> = vec![];

        let white_ish = Arc::new(Lambertian {
            albedo: SolidColor::arc_from(Color64::gray(0.73)),
        });

        let num_of_spheres_in_box = 1000;

        (0..num_of_spheres_in_box).for_each(|_| {
            box_of_spheres.push(Arc::new(Sphere {
                center: Point64(vector3::rand_range(0., 165.)),
                radius: 10.,
                material: white_ish.clone(),
            }));
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
            hittable: Arc::new(HittableVec {
                hittables: vec![
                    // floor
                    BoundedVolumeHierarchy::create_bvh_arc(&mut boxes, 0., 1.),
                    // light
                    Arc::new(AxisAlignedRect {
                        material: Arc::new(DiffuseLight::new(Color64::new(7., 7., 7.))),
                        min: (123.0, 147.0),
                        max: (423.0, 412.0),
                        axis_value: 554.0,
                        axis_alignment: AxisAlignment::Y,
                    }),
                    // moving sphere
                    Arc::new(MovingSphere {
                        center0: Point64::new(400., 400., 200.),
                        center1: Point64::new(430., 400., 200.),
                        radius: 50.0,
                        material: Arc::new(Lambertian {
                            albedo: SolidColor::arc_from(Color64::new(0.7, 0.3, 0.1)),
                        }),
                        time0: 0.0,
                        time1: 1.0,
                    }),
                    // dielectric sphere
                    Arc::new(Sphere {
                        center: Point64::new(260., 150., 45.),
                        radius: 50.0,
                        material: Arc::new(Dielectric {
                            index_of_refraction: 1.5,
                        }),
                    }),
                    // metal sphere
                    Arc::new(Sphere {
                        center: Point64::new(0., 150., 145.),
                        radius: 50.0,
                        material: Arc::new(Metal {
                            albedo: Color64::new(0.8, 0.8, 0.9),
                            fuzz: 1.0,
                        }),
                    }),
                    // TODO blue subsurface reflection sphere

                    // earth
                    Arc::new(Sphere {
                        center: Point64::new(400., 200., 400.),
                        radius: 100.,
                        material: Arc::new(Lambertian {
                            albedo: Arc::new(ImageTexture::new(
                                "resources/earthmap.jpg".to_string(),
                            )),
                        }),
                    }),
                    // Perlin noise sphere
                    Arc::new(Sphere {
                        center: Point64::new(220., 280., 300.),
                        radius: 80.,
                        material: Arc::new(Lambertian {
                            albedo: Arc::new(Noise {
                                noise_gen: PerlinGenerator::new(),
                                scale: 0.1,
                                noise_type: NoiseType::Perlin,
                            }),
                        }),
                    }),
                    // rotated/translated box of spheres
                    // TODO rotation
                    Arc::new(Translation {
                        hittable: BoundedVolumeHierarchy::create_bvh_arc(
                            &mut box_of_spheres,
                            0.0,
                            1.0,
                        ),
                        offset: Vector3::new(-100., 270., 395.),
                    }),
                ],
            }),
        }
    }
}
