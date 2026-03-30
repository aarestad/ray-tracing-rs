use crate::camera::{Camera, CameraRecipe};
use crate::data::color64::{BLACK, Color64, LIGHT_BLUE};
use crate::data::point64::Point64;
use crate::data::vector3::{rand_range, random_in_unit_cube};
use crate::hittables::Hittable;
use crate::hittables::axis_aligned_rect::AxisAlignedRect;
use crate::hittables::axis_aligned_rect::AxisAlignment::{self, X, Y, Z};
use crate::hittables::bounded_volume_hierarchy::BoundedVolumeHierarchy;
use crate::hittables::rotation::Rotation;
use crate::hittables::cuboid::Cuboid;
use crate::hittables::hittable_vec::HittableVec;
use crate::hittables::moving_sphere::MovingSphere;
use crate::hittables::sphere::Sphere;
use crate::hittables::translation::Translation;
use crate::materials::Material;
use crate::materials::dielectric::Dielectric;
use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::textures::Texture;
use crate::textures::image::ImageTexture;
use crate::textures::noise::NoiseType::Marble;
use crate::textures::noise::{Noise, NoiseType};
use crate::textures::perlin::PerlinGenerator;
use crate::util::obj::{load_obj_triangles, obj_mesh_axis_bounds};
use nalgebra::Vector3;
use rand::Rng;
use std::ops::Range;
use std::path::Path;

pub(crate) struct World {
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub background_color: Color64,
    pub camera: Camera,
    pub camera_target: Point64,
    pub camera_v_up: Vector3<f64>,
    pub camera_vfov_deg: f64,
    pub camera_aperture: f64,
    pub camera_focus_distance: f64,
    pub camera_exposure_time: Range<f64>,
    pub hittable: Hittable,
    /// Y coordinate of the ground plane, if any. The interactive camera will not
    /// allow the viewpoint to drop below this level.
    pub ground_y: Option<f64>,
}

const DEFAULT_LOOK_FROM: Point64 = Point64::new(13., 2., 3.);
const DEFAULT_LOOK_AT: Point64 = Point64::new(0., 0., 0.);
const DEFAULT_SAMPLES_PER_PIXEL: u32 = 100;
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
        let checker_pattern = Texture::Checker {
            odd: Box::new(Texture::solid(Color64::new(0.2, 0.3, 0.1))),
            even: Box::new(Texture::solid(Color64::new(0.9, 0.9, 0.9))),
        };

        let mut hittables: Vec<Hittable> = vec![Hittable::Sphere(Sphere {
            center: Point64::new(0., -1000., 0.),
            radius: 1000.,
            material: Material::Lambertian(Lambertian {
                albedo: checker_pattern,
            }),
        })];

        let glass = Material::Dielectric(Dielectric {
            index_of_refraction: 1.5,
        });

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
                        hittables.push(Hittable::MovingSphere(MovingSphere {
                            center0: center,
                            center1: Point64(center.0 + Vector3::new(0., rng.random(), 0.)),
                            time0: 0.,
                            time1: 1.,
                            radius: 0.2,
                            material: Material::Lambertian(Lambertian {
                                albedo: Texture::solid(Color64(
                                    random_in_unit_cube().component_mul(&random_in_unit_cube()),
                                )),
                            }),
                        }));
                    } else if choose_mat < 0.8 {
                        // 70% stationary Lambertian spheres
                        hittables.push(Hittable::Sphere(Sphere {
                            center,
                            radius: 0.2,
                            material: Material::Lambertian(Lambertian {
                                albedo: Texture::solid(Color64(
                                    random_in_unit_cube().component_mul(&random_in_unit_cube()),
                                )),
                            }),
                        }));
                    } else if choose_mat < 0.95 {
                        // 15% metal spheres
                        hittables.push(Hittable::Sphere(Sphere {
                            center,
                            radius: 0.2,
                            material: Material::Metal(Metal {
                                albedo: Color64(rand_range(0.5, 1.)),
                                fuzz: rng.random_range(0.0..0.5),
                            }),
                        }));
                    } else {
                        // 5% glass
                        hittables.push(Hittable::Sphere(Sphere {
                            center,
                            radius: 0.2,
                            material: glass.clone(),
                        }));
                    }
                }
            }
        }

        hittables.push(Hittable::Sphere(Sphere {
            center: Point64::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: glass,
        }));

        hittables.push(Hittable::Sphere(Sphere {
            center: Point64::new(-4.0, 1.0, 0.0),
            radius: 1.0,
            material: Material::Lambertian(Lambertian {
                albedo: Texture::solid(Color64::new(0.4, 0.2, 0.1)),
            }),
        }));

        hittables.push(Hittable::Sphere(Sphere {
            center: Point64::new(4.0, 1.0, 0.0),
            radius: 1.0,
            material: Material::Metal(Metal {
                albedo: Color64::new(0.7, 0.6, 0.5),
                fuzz: 0.,
            }),
        }));

        let hittable = if use_bvh {
            BoundedVolumeHierarchy::create_bvh(&mut hittables, 0.0, 1.0)
        } else {
            Hittable::HittableVec(HittableVec { hittables })
        };

        let aspect = DEFAULT_IMAGE_WIDTH as f64 / DEFAULT_IMAGE_HEIGHT as f64;
        let recipe = CameraRecipe::new(
            DEFAULT_LOOK_FROM,
            DEFAULT_LOOK_AT,
            DEFAULT_VUP,
            20.0,
            aspect,
            0.1,
            DEFAULT_FOCUS_DISTANCE,
            DEFAULT_EXPOSURE_TIME,
        );

        World {
            image_width: DEFAULT_IMAGE_WIDTH,
            image_height: DEFAULT_IMAGE_HEIGHT,
            samples_per_pixel: DEFAULT_SAMPLES_PER_PIXEL,
            background_color: LIGHT_BLUE,
            camera: recipe.camera,
            camera_target: recipe.look_at,
            camera_v_up: recipe.v_up,
            camera_vfov_deg: recipe.vfov_deg,
            camera_aperture: recipe.aperture,
            camera_focus_distance: recipe.focus_distance,
            camera_exposure_time: recipe.exposure_time,
            hittable,
            ground_y: Some(0.0),
        }
    }

    pub fn two_spheres() -> World {
        let material = Material::Lambertian(Lambertian {
            albedo: Texture::Checker {
                odd: Box::new(Texture::solid(Color64::new(0.2, 0.3, 0.1))),
                even: Box::new(Texture::solid(Color64::new(0.9, 0.9, 0.9))),
            },
        });

        let hittable = Hittable::HittableVec(HittableVec {
            hittables: vec![
                Hittable::Sphere(Sphere {
                    center: Point64::new(0., -10., 0.),
                    radius: 10.,
                    material: material.clone(),
                }),
                Hittable::Sphere(Sphere {
                    center: Point64::new(0., 10., 0.),
                    radius: 10.,
                    material,
                }),
            ],
        });

        let aspect = DEFAULT_IMAGE_WIDTH as f64 / DEFAULT_IMAGE_HEIGHT as f64;
        let recipe = CameraRecipe::new(
            DEFAULT_LOOK_FROM,
            DEFAULT_LOOK_AT,
            DEFAULT_VUP,
            20.0,
            aspect,
            0.0,
            DEFAULT_FOCUS_DISTANCE,
            DEFAULT_EXPOSURE_TIME,
        );

        World {
            image_width: DEFAULT_IMAGE_WIDTH,
            image_height: DEFAULT_IMAGE_HEIGHT,
            samples_per_pixel: DEFAULT_SAMPLES_PER_PIXEL,
            background_color: LIGHT_BLUE,
            camera: recipe.camera,
            camera_target: recipe.look_at,
            camera_v_up: recipe.v_up,
            camera_vfov_deg: recipe.vfov_deg,
            camera_aperture: recipe.aperture,
            camera_focus_distance: recipe.focus_distance,
            camera_exposure_time: recipe.exposure_time,
            hittable,
            ground_y: Some(0.0),
        }
    }

    pub fn two_perlin_spheres(noise_type: NoiseType) -> World {
        let material = Material::Lambertian(Lambertian {
            albedo: Texture::Noise(Box::new(Noise {
                noise_gen: PerlinGenerator::new(),
                scale: 4.,
                noise_type,
            })),
        });

        let hittable = Hittable::HittableVec(HittableVec {
            hittables: vec![
                Hittable::Sphere(Sphere {
                    center: Point64::new(0., -1000., 0.),
                    radius: 1000.,
                    material: material.clone(),
                }),
                Hittable::Sphere(Sphere {
                    center: Point64::new(0., 2., 0.),
                    radius: 2.,
                    material,
                }),
            ],
        });

        let aspect = DEFAULT_IMAGE_WIDTH as f64 / DEFAULT_IMAGE_HEIGHT as f64;
        let recipe = CameraRecipe::new(
            DEFAULT_LOOK_FROM,
            DEFAULT_LOOK_AT,
            DEFAULT_VUP,
            20.0,
            aspect,
            DEFAULT_APERTURE,
            DEFAULT_FOCUS_DISTANCE,
            DEFAULT_EXPOSURE_TIME,
        );

        World {
            image_width: DEFAULT_IMAGE_WIDTH,
            image_height: DEFAULT_IMAGE_HEIGHT,
            samples_per_pixel: DEFAULT_SAMPLES_PER_PIXEL,
            background_color: LIGHT_BLUE,
            camera: recipe.camera,
            camera_target: recipe.look_at,
            camera_v_up: recipe.v_up,
            camera_vfov_deg: recipe.vfov_deg,
            camera_aperture: recipe.aperture,
            camera_focus_distance: recipe.focus_distance,
            camera_exposure_time: recipe.exposure_time,
            hittable,
            ground_y: Some(0.0),
        }
    }

    pub fn earth() -> World {
        let hittable = Hittable::Sphere(Sphere {
            center: Point64::new(0., 0., 0.),
            radius: 2.,
            material: Material::Lambertian(Lambertian {
                albedo: Texture::Image(ImageTexture::new(
                    "resources/earthmap.jpg".into(),
                )),
            }),
        });

        let aspect = DEFAULT_IMAGE_WIDTH as f64 / DEFAULT_IMAGE_HEIGHT as f64;
        let recipe = CameraRecipe::new(
            DEFAULT_LOOK_FROM,
            DEFAULT_LOOK_AT,
            DEFAULT_VUP,
            20.0,
            aspect,
            DEFAULT_APERTURE,
            DEFAULT_FOCUS_DISTANCE,
            DEFAULT_EXPOSURE_TIME,
        );

        World {
            image_width: DEFAULT_IMAGE_WIDTH,
            image_height: DEFAULT_IMAGE_HEIGHT,
            samples_per_pixel: DEFAULT_SAMPLES_PER_PIXEL,
            background_color: LIGHT_BLUE,
            camera: recipe.camera,
            camera_target: recipe.look_at,
            camera_v_up: recipe.v_up,
            camera_vfov_deg: recipe.vfov_deg,
            camera_aperture: recipe.aperture,
            camera_focus_distance: recipe.focus_distance,
            camera_exposure_time: recipe.exposure_time,
            hittable,
            ground_y: None,
        }
    }

    pub fn simple_light() -> World {
        let material = Material::Lambertian(Lambertian {
            albedo: Texture::Noise(Box::new(Noise {
                noise_gen: PerlinGenerator::new(),
                scale: 4.,
                noise_type: Marble,
            })),
        });

        let light = DiffuseLight::new(Color64::new(4., 4., 4.));

        let hittable = Hittable::HittableVec(HittableVec {
            hittables: vec![
                Hittable::Sphere(Sphere {
                    center: Point64::new(0., -1000., 0.),
                    radius: 1000.,
                    material: material.clone(),
                }),
                Hittable::Sphere(Sphere {
                    center: Point64::new(0., 2., 0.),
                    radius: 2.,
                    material,
                }),
                Hittable::AxisAlignedRect(AxisAlignedRect {
                    material: Material::DiffuseLight(light),
                    min: (3., 1.),
                    max: (5., 3.),
                    axis_value: -2.,
                    axis_alignment: Z,
                }),
            ],
        });

        let aspect = DEFAULT_IMAGE_WIDTH as f64 / DEFAULT_IMAGE_HEIGHT as f64;
        let recipe = CameraRecipe::new(
            Point64::new(26., 3., 6.),
            Point64::new(0., 2., 0.),
            DEFAULT_VUP,
            20.0,
            aspect,
            DEFAULT_APERTURE,
            DEFAULT_FOCUS_DISTANCE,
            DEFAULT_EXPOSURE_TIME,
        );

        World {
            image_width: DEFAULT_IMAGE_WIDTH,
            image_height: DEFAULT_IMAGE_HEIGHT,
            samples_per_pixel: 400,
            background_color: BLACK,
            camera: recipe.camera,
            camera_target: recipe.look_at,
            camera_v_up: recipe.v_up,
            camera_vfov_deg: recipe.vfov_deg,
            camera_aperture: recipe.aperture,
            camera_focus_distance: recipe.focus_distance,
            camera_exposure_time: recipe.exposure_time,
            hittable,
            ground_y: Some(0.0),
        }
    }

    pub fn cornell_box() -> World {
        let red_material = Lambertian {
            albedo: Texture::solid(Color64::new(0.65, 0.05, 0.05)),
        };

        let gray_material = Material::Lambertian(Lambertian {
            albedo: Texture::solid(Color64::gray(0.73)),
        });

        let green_material = Lambertian {
            albedo: Texture::solid(Color64::new(0.12, 0.45, 0.15)),
        };

        let light_source = DiffuseLight::new(Color64::gray(15.));

        let hittable = Hittable::HittableVec(HittableVec {
            hittables: vec![
                Hittable::AxisAlignedRect(AxisAlignedRect {
                    material: Material::Lambertian(green_material),
                    min: (0., 0.),
                    max: (555., 555.),
                    axis_value: 555.,
                    axis_alignment: X,
                }),
                Hittable::AxisAlignedRect(AxisAlignedRect {
                    material: Material::Lambertian(red_material),
                    min: (0., 0.),
                    max: (555., 555.),
                    axis_value: 0.,
                    axis_alignment: X,
                }),
                Hittable::AxisAlignedRect(AxisAlignedRect {
                    material: Material::DiffuseLight(light_source),
                    min: (213., 227.),
                    max: (343., 332.),
                    axis_value: 554.,
                    axis_alignment: Y,
                }),
                Hittable::AxisAlignedRect(AxisAlignedRect {
                    material: gray_material.clone(),
                    min: (0., 0.),
                    max: (555., 555.),
                    axis_value: 0.,
                    axis_alignment: Y,
                }),
                Hittable::AxisAlignedRect(AxisAlignedRect {
                    material: gray_material.clone(),
                    min: (0., 0.),
                    max: (555., 555.),
                    axis_value: 555.,
                    axis_alignment: Y,
                }),
                Hittable::AxisAlignedRect(AxisAlignedRect {
                    material: gray_material.clone(),
                    min: (0., 0.),
                    max: (555., 555.),
                    axis_value: 555.,
                    axis_alignment: Z,
                }),
                Hittable::Translation(Translation {
                    hittable: Box::new(Hittable::Cuboid(Cuboid::new(
                        Point64::new(0., 0., 0.),
                        Point64::new(165., 330., 165.),
                        gray_material.clone(),
                    ))),
                    offset: Vector3::new(265., 0., 295.),
                }),
                Hittable::Translation(Translation {
                    hittable: Box::new(Hittable::Cuboid(Cuboid::new(
                        Point64::new(0., 0., 0.),
                        Point64::new(165., 165., 165.),
                        gray_material,
                    ))),
                    offset: Vector3::new(130., 0., 65.),
                }),
            ],
        });

        let recipe = CameraRecipe::new(
            Point64::new(278., 278., -800.),
            Point64::new(278., 278., 0.),
            DEFAULT_VUP,
            DEFAULT_VFOV_DEG,
            1.,
            DEFAULT_APERTURE,
            DEFAULT_FOCUS_DISTANCE,
            DEFAULT_EXPOSURE_TIME,
        );

        World {
            image_width: 600,
            image_height: 600,
            samples_per_pixel: DEFAULT_SAMPLES_PER_PIXEL,
            background_color: BLACK,
            camera: recipe.camera,
            camera_target: recipe.look_at,
            camera_v_up: recipe.v_up,
            camera_vfov_deg: recipe.vfov_deg,
            camera_aperture: recipe.aperture,
            camera_focus_distance: recipe.focus_distance,
            camera_exposure_time: recipe.exposure_time,
            hittable,
            ground_y: None,
        }
    }

    pub fn final_scene() -> World {
        let mut rng = rand::rng();

        let ground = Material::Lambertian(Lambertian {
            albedo: Texture::solid(Color64::new(0.48, 0.83, 0.53)),
        });

        // create the floor boxes
        let mut boxes: Vec<Hittable> = vec![];

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

                boxes.push(Hittable::Cuboid(Cuboid::new(
                    Point64::new(x0, y0, z0),
                    Point64::new(x1, y1, z1),
                    ground.clone(),
                )));
            });
        });

        // create the box of spheres
        let mut box_of_spheres: Vec<Hittable> = vec![];

        let white_ish = Material::Lambertian(Lambertian {
            albedo: Texture::solid(Color64::gray(0.73)),
        });

        let num_of_spheres_in_box = 1000;

        (0..num_of_spheres_in_box).for_each(|_| {
            box_of_spheres.push(Hittable::Sphere(Sphere {
                center: Point64(rand_range(0., 165.)),
                radius: 10.,
                material: white_ish.clone(),
            }));
        });

        let recipe = CameraRecipe::new(
            Point64::new(478., 278., -600.),
            Point64::new(278., 278., 0.),
            DEFAULT_VUP,
            DEFAULT_VFOV_DEG,
            1.,
            DEFAULT_APERTURE,
            DEFAULT_FOCUS_DISTANCE,
            DEFAULT_EXPOSURE_TIME,
        );

        World {
            image_width: 800,
            image_height: 800,
            samples_per_pixel: 100,
            background_color: BLACK,
            camera: recipe.camera,
            camera_target: recipe.look_at,
            camera_v_up: recipe.v_up,
            camera_vfov_deg: recipe.vfov_deg,
            camera_aperture: recipe.aperture,
            camera_focus_distance: recipe.focus_distance,
            camera_exposure_time: recipe.exposure_time,
            ground_y: Some(0.0),
            hittable: {
                let mut scene: Vec<Hittable> = vec![
                    // floor
                    BoundedVolumeHierarchy::create_bvh(&mut boxes, 0., 1.),
                    // light
                    Hittable::AxisAlignedRect(AxisAlignedRect {
                        material: Material::DiffuseLight(DiffuseLight::new(
                            Color64::new(7., 7., 7.),
                        )),
                        min: (123.0, 147.0),
                        max: (423.0, 412.0),
                        axis_value: 554.0,
                        axis_alignment: Y,
                    }),
                    // moving sphere
                    Hittable::MovingSphere(MovingSphere {
                        center0: Point64::new(400., 400., 200.),
                        center1: Point64::new(430., 400., 200.),
                        radius: 50.0,
                        material: Material::Lambertian(Lambertian {
                            albedo: Texture::solid(Color64::new(0.7, 0.3, 0.1)),
                        }),
                        time0: 0.0,
                        time1: 1.0,
                    }),
                    // dielectric sphere
                    Hittable::Sphere(Sphere {
                        center: Point64::new(260., 150., 45.),
                        radius: 50.0,
                        material: Material::Dielectric(Dielectric {
                            index_of_refraction: 1.5,
                        }),
                    }),
                    // metal sphere
                    Hittable::Sphere(Sphere {
                        center: Point64::new(0., 150., 145.),
                        radius: 50.0,
                        material: Material::Metal(Metal {
                            albedo: Color64::new(0.8, 0.8, 0.9),
                            fuzz: 1.0,
                        }),
                    }),
                    // TODO blue subsurface reflection sphere

                    // earth
                    Hittable::Sphere(Sphere {
                        center: Point64::new(400., 200., 400.),
                        radius: 100.,
                        material: Material::Lambertian(Lambertian {
                            albedo: Texture::Image(ImageTexture::new(
                                "resources/earthmap.jpg".to_string(),
                            )),
                        }),
                    }),
                    // Perlin noise sphere
                    Hittable::Sphere(Sphere {
                        center: Point64::new(220., 280., 300.),
                        radius: 80.,
                        material: Material::Lambertian(Lambertian {
                            albedo: Texture::Noise(Box::new(Noise {
                                noise_gen: PerlinGenerator::new(),
                                scale: 0.1,
                                noise_type: NoiseType::Perlin,
                            })),
                        }),
                    }),
                    // rotated/translated box of spheres
                    // TODO rotation
                    Hittable::Translation(Translation {
                        hittable: Box::new(BoundedVolumeHierarchy::create_bvh(
                            &mut box_of_spheres,
                            0.0,
                            1.0,
                        )),
                        offset: Vector3::new(-100., 270., 395.),
                    }),
                ];
                BoundedVolumeHierarchy::create_bvh(&mut scene, 0., 1.)
            },
        }
    }

    /// Utah teapot mesh loaded from `resources/teapot.obj` — several instances with different materials.
    pub fn utah_teapots() -> World {
        let teapot_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/teapot.obj");
        let mini_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/minicooper.obj");

        let teapot_scale = 0.38_f64;
        let teapot_bounds = obj_mesh_axis_bounds(&teapot_path)
            .unwrap_or_else(|e| panic!("failed to read bounds for {}: {e}", teapot_path.display()));
        // Place mesh so its lowest vertex lies on the ground (y ≈ 0), plus a small lift so the
        // large ground sphere does not clip the base at off-center x positions.
        const GROUND_LIFT: f64 = 0.04;
        let sit_teapot = -teapot_bounds.y_min * teapot_scale + GROUND_LIFT;
        let teapot_height_world = teapot_bounds.height() * teapot_scale;

        let mini_bounds = obj_mesh_axis_bounds(&mini_path)
            .unwrap_or_else(|e| panic!("failed to read bounds for {}: {e}", mini_path.display()));
        let car_scale = teapot_height_world / mini_bounds.height();
        let sit_car = -mini_bounds.y_min * car_scale + GROUND_LIFT;
        let car_z = -0.5 * (mini_bounds.z_min + mini_bounds.z_max) * car_scale;

        let checker = Material::Lambertian(Lambertian {
            albedo: Texture::Checker {
                odd: Box::new(Texture::solid(Color64::new(0.15, 0.25, 0.12))),
                even: Box::new(Texture::solid(Color64::new(0.85, 0.85, 0.88))),
            },
        });
        let copper = Material::Metal(Metal {
            albedo: Color64::new(0.95, 0.64, 0.54),
            fuzz: 0.15,
        });
        let noise = Material::Lambertian(Lambertian {
            albedo: Texture::Noise(Box::new(Noise {
                noise_gen: PerlinGenerator::new(),
                scale: 2.5,
                noise_type: NoiseType::Turbulence,
            })),
        });
        let earth = Material::Lambertian(Lambertian {
            albedo: Texture::Image(ImageTexture::new(
                "resources/earthmap.jpg".into(),
            )),
        });
        let glass = Material::Dielectric(Dielectric {
            index_of_refraction: 1.5,
        });
        let silver = Material::Metal(Metal {
            albedo: Color64::new(0.92, 0.93, 0.96),
            fuzz: 0.04,
        });

        let mut hittables: Vec<Hittable> = vec![Hittable::Sphere(Sphere {
            center: Point64::new(0., -1000., 0.),
            radius: 1000.,
            material: Material::Lambertian(Lambertian {
                albedo: Texture::solid(Color64::new(0.45, 0.45, 0.48)),
            }),
        })];

        // Five teapots and one Mini Cooper; spacing 3 on each side of x = 0 (car in the middle).
        // Each teapot is built at the origin (grounded), rotated, then translated along x.
        let teapot_placements: [(f64, Material, AxisAlignment, f64); 5] = [
            (-9.0, checker.clone(), X, 18_f64.to_radians()),
            (-6.0, copper.clone(), Y, 35_f64.to_radians()),
            (-3.0, noise.clone(), Z, 15_f64.to_radians()),
            (3.0, earth.clone(), Y, (-25_f64).to_radians()),
            (6.0, glass.clone(), Z, (-42_f64).to_radians()),
        ];

        for (x, mat, axis, angle) in teapot_placements {
            let mut tris = load_obj_triangles(
                &teapot_path,
                mat,
                teapot_scale,
                Vector3::new(0., sit_teapot, 0.0),
            )
            .unwrap_or_else(|e| panic!("failed to load {}: {e}", teapot_path.display()));
            let bvh = BoundedVolumeHierarchy::create_bvh(&mut tris, 0., 1.);
            let rotated = Hittable::Rotation(Rotation::new(
                Box::new(bvh), axis, angle, 0., 1.,
            ));
            hittables.push(Hittable::Translation(Translation {
                hittable: Box::new(rotated),
                offset: Vector3::new(x, 0., 0.),
            }));
        }

        // Car: wheels on ground; rotate +90° about Z so the long axis faces the viewer's right (+X).
        let mut car_tris = load_obj_triangles(
            &mini_path,
            silver,
            car_scale,
            Vector3::new(0.0, sit_car, car_z),
        )
        .unwrap_or_else(|e| panic!("failed to load {}: {e}", mini_path.display()));
        let car_bvh = BoundedVolumeHierarchy::create_bvh(&mut car_tris, 0., 1.);
        hittables.push(Hittable::Rotation(Rotation::new(
            Box::new(car_bvh),
            Z,
            std::f64::consts::FRAC_PI_2,
            0.,
            1.,
        )));

        let hittable = BoundedVolumeHierarchy::create_bvh(&mut hittables, 0., 1.);

        let aspect = DEFAULT_IMAGE_WIDTH as f64 / DEFAULT_IMAGE_HEIGHT as f64;
        let recipe = CameraRecipe::new(
            Point64::new(0., 3.2, 17.5),
            Point64::new(0., 0.9, 0.),
            DEFAULT_VUP,
            40.,
            aspect,
            DEFAULT_APERTURE,
            DEFAULT_FOCUS_DISTANCE,
            DEFAULT_EXPOSURE_TIME,
        );

        World {
            image_width: DEFAULT_IMAGE_WIDTH,
            image_height: DEFAULT_IMAGE_HEIGHT,
            samples_per_pixel: DEFAULT_SAMPLES_PER_PIXEL,
            background_color: LIGHT_BLUE,
            camera: recipe.camera,
            camera_target: recipe.look_at,
            camera_v_up: recipe.v_up,
            camera_vfov_deg: recipe.vfov_deg,
            camera_aperture: recipe.aperture,
            camera_focus_distance: recipe.focus_distance,
            camera_exposure_time: recipe.exposure_time,
            hittable,
            ground_y: Some(0.0),
        }
    }
}
