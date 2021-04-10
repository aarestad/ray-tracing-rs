use crate::data::color64::Color64;
use crate::data::point64::Point64;
use crate::data::vec3_64::Vec3_64;
use crate::hittables::bounded_volume_hierarchy::BoundedVolumeHierarchy;
use crate::hittables::hittable_vec::HittableVec;
use crate::hittables::moving_sphere::MovingSphere;
use crate::hittables::sphere::Sphere;
use crate::hittables::Hittable;
use crate::materials::dielectric::Dielectric;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::textures::checker::Checker;
use crate::textures::noise::{Noise, NoiseType};
use crate::textures::perlin::PerlinGenerator;
use crate::textures::solid_color::SolidColor;
use rand::Rng;
use std::sync::Arc;

pub fn random_world(create_little_spheres: bool, use_bvh: bool) -> Arc<dyn Hittable> {
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
                                    Vec3_64::random_in_unit_cube() * Vec3_64::random_in_unit_cube(),
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
                                    Vec3_64::random_in_unit_cube() * Vec3_64::random_in_unit_cube(),
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

    if use_bvh {
        BoundedVolumeHierarchy::create_bvh_arc(&mut hittables, 0.0, 1.0)
    } else {
        Arc::new(HittableVec { hittables })
    }
}

pub fn two_spheres() -> Arc<dyn Hittable> {
    let material = Arc::from(Lambertian {
        albedo: Arc::from(Checker {
            odd: SolidColor::arc_from(Color64::new(0.2, 0.3, 0.1)),
            even: SolidColor::arc_from(Color64::new(0.9, 0.9, 0.9)),
        }),
    });

    Arc::new(HittableVec {
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
    })
}

pub fn two_perlin_spheres(noise_type: NoiseType) -> Arc<dyn Hittable> {
    let material = Arc::from(Lambertian {
        albedo: Arc::from(Noise {
            noise_gen: PerlinGenerator::new(),
            scale: 4.,
            noise_type,
        }),
    });

    Arc::new(HittableVec {
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
    })
}
