use crate::data::color64::{BLACK, Color64};
use crate::data::point64::Point64;
use crate::data::ray::Ray;
use crate::data::vector3::{near_zero, random_in_unit_sphere, reflect, refract};
use crate::hittables::HitRecord;
use rand_distr::num_traits::Inv;
use crate::textures::Textures;

pub mod dielectric;

pub struct ScatterRecord {
    #[allow(dead_code)]
    pub hit_record: HitRecord,
    pub attenuation: Color64,
    pub scattered: Ray,
}

#[derive(Clone)]
pub enum Materials {
    Dielectric(f64 /* index_of_refraction */),
    DiffuseLight(Textures /* emitter */),
    Lambertian(Textures /* albedo */),
    Metal(Color64 /* albedo */, f64 /* fuzz */),
}

impl Materials {
    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        match self {
            Materials::Dielectric(index_of_refraction) => {
                let refraction_ratio = if hit_record.front_face {
                    index_of_refraction.inv()
                } else {
                    *index_of_refraction
                };

                let unit_direction = ray_in.direction.0.normalize();

                let cos_theta = -unit_direction.dot(&hit_record.normal.0).min(1.);
                let sin_theta = (1. - cos_theta.powi(2)).sqrt();
                let cannot_refract = refraction_ratio * sin_theta > 1.;

                let direction =
                    if cannot_refract || dielectric::reflectance(cos_theta, refraction_ratio) > rand::random() {
                        reflect(&unit_direction, &hit_record.normal.0)
                    } else {
                        refract(&unit_direction, &hit_record.normal.0, refraction_ratio)
                    };

                Some(ScatterRecord {
                    hit_record: hit_record.clone(),
                    attenuation: Color64::new(1., 1., 1.),
                    scattered: Ray {
                        origin: hit_record.location,
                        direction: Point64(direction),
                        exposure_time: ray_in.exposure_time,
                    },
                })
            },
            Materials::Lambertian(albedo) => {
                let scatter_direction = hit_record.normal.0 + random_in_unit_sphere();

                Some(ScatterRecord {
                    hit_record: hit_record.clone(),
                    attenuation: albedo
                        .value(hit_record.u, hit_record.v, &hit_record.location),
                    scattered: Ray {
                        origin: hit_record.location,
                        direction: if near_zero(&scatter_direction) {
                            hit_record.normal
                        } else {
                            Point64(scatter_direction)
                        },
                        exposure_time: ray_in.exposure_time,
                    },
                })
            },
            Materials::Metal(albedo, fuzz) => {
                let reflected = reflect(&ray_in.direction.0.normalize(), &hit_record.normal.0);

                if reflected.dot(&hit_record.normal.0) > 0. {
                    Some(ScatterRecord {
                        hit_record: hit_record.clone(),
                        attenuation: *albedo,
                        scattered: Ray {
                            origin: hit_record.location,
                            direction: Point64(reflected + *fuzz * random_in_unit_sphere()),
                            exposure_time: ray_in.exposure_time,
                        },
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn emitted(&self, u: f64, v: f64, point: &Point64) -> Color64 {
        match self {
            Materials::DiffuseLight(emitter) => emitter.value(u, v, point),
            _ => BLACK,
        }
    }
}
