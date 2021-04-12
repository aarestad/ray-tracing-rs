use crate::camera::Camera;
use crate::data::color64::{Color64, BLACK, LIGHT_BLUE};
use crate::data::point64::Point64;
use crate::data::vec3_64::Vec3_64;
use crate::hittables::axis_aligned_rect::AxisAlignedRect;
use crate::hittables::axis_aligned_rect::AxisAlignment::{X, Y, Z};
use crate::hittables::bounded_volume_hierarchy::BoundedVolumeHierarchy;
use crate::hittables::hittable_vec::HittableVec;
use crate::hittables::moving_sphere::MovingSphere;
use crate::hittables::sphere::Sphere;
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
const DEFAULT_VUP: Vec3_64 = Vec3_64(0., 1., 0.);
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

    pub fn random_world(create_little_spheres: bool, use_bvh: bool) -> World {
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

        if create_little_spheres {
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
                                center1: Point64(*center + Vec3_64(0., rng.gen(), 0.)),
                                time0: 0.,
                                time1: 1.,
                                radius: 0.2,
                                material: Arc::new(Lambertian {
                                    albedo: SolidColor::arc_from(Color64(
                                        Vec3_64::random_in_unit_cube()
                                            * Vec3_64::random_in_unit_cube(),
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
                                        Vec3_64::random_in_unit_cube()
                                            * Vec3_64::random_in_unit_cube(),
                                    )),
                                }),
                            }));
                        } else if choose_mat < 0.95 {
                            // 15% metal spheres
                            hittables.push(Arc::new(Sphere {
                                center,
                                radius: 0.2,
                                material: Arc::new(Metal {
                                    albedo: Color64(Vec3_64::rand_range(0.5, 1.)),
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
                    material: gray_material,
                    min: (0., 0.),
                    max: (555., 555.),
                    axis_value: 555.,
                    axis_alignment: Z,
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
}
