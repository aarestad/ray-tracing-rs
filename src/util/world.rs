use crate::data::color64::Color64;
use crate::data::point64::Point64;
use crate::data::vec3_64::Vec3_64;
use crate::hittables::hittable_vec::HittableVec;
use crate::hittables::moving_sphere::MovingSphere;
use crate::hittables::sphere::Sphere;
use crate::hittables::Hittable;
use crate::materials::dielectric::Dielectric;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::textures::solid_color::SolidColor;
use rand::Rng;
use std::sync::Arc;

pub fn create_world(create_little_spheres: bool) -> Arc<dyn Hittable + Send + Sync> {
    let mut hittables: Vec<Box<dyn Hittable>> = vec![Box::new(Sphere {
        center: Point64::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian {
            albedo: SolidColor::arc_from(Color64::new(0.5, 0.5, 0.5)),
        }),
    })];

    let glass = Arc::new(Dielectric {
        index_of_refraction: 1.5,
    });

    if create_little_spheres {
        let mut rng = rand::thread_rng();

        let reference_point = Point64::new(4.0, 0.2, 0.0);

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
                        hittables.push(Box::new(MovingSphere {
                            center0: center,
                            center1: Point64(*center + Vec3_64(0.0, rng.gen(), 0.0)),
                            time0: 0.0,
                            time1: 1.0,
                            radius: 0.2,
                            material: Arc::new(Lambertian {
                                albedo: SolidColor::arc_from(Color64(
                                    Vec3_64::random_in_unit_cube() * Vec3_64::random_in_unit_cube(),
                                )),
                            }),
                        }));
                    } else if choose_mat < 0.8 {
                        // 70% stationary Lambertian spheres
                        hittables.push(Box::new(Sphere {
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
                        hittables.push(Box::new(Sphere {
                            center,
                            radius: 0.2,
                            material: Arc::new(Metal {
                                albedo: Color64(Vec3_64::rand_range(0.5, 1.0)),
                                fuzz: rng.gen_range(0.0..0.5),
                            }),
                        }));
                    } else {
                        // 5% glass
                        hittables.push(Box::new(Sphere {
                            center,
                            radius: 0.2,
                            material: glass.clone(),
                        }));
                    }
                }
            }
        }
    }

    hittables.push(Box::new(Sphere {
        center: Point64::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: glass,
    }));

    hittables.push(Box::new(Sphere {
        center: Point64::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Lambertian {
            albedo: SolidColor::arc_from(Color64::new(0.4, 0.2, 0.1)),
        }),
    }));

    hittables.push(Box::new(Sphere {
        center: Point64::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Metal {
            albedo: Color64::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    }));

    Arc::new(HittableVec { hittables })
}
