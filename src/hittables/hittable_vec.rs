use crate::data::interval::Interval;
use crate::data::ray::Ray;
use crate::hittables::axis_aligned_bounding_box::AxisAlignedBoundingBox;
use crate::hittables::{HitRecord, Hittable};

#[derive(Clone)]
pub struct HittableVec {
    pub hittables: Vec<Hittable>,
}

impl HittableVec {
    pub fn bounding_box(&self, time0: f64, time1: f64) -> Option<AxisAlignedBoundingBox> {
        if self.hittables.is_empty() {
            return None;
        };

        self.hittables.iter().fold(
            self.hittables[0].bounding_box(time0, time1),
            |acc, hittable| Some(acc?.surrounding_box_with(&hittable.bounding_box(time0, time1)?)),
        )
    }

    pub fn is_hit_by(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut winner: Option<HitRecord> = None;

        for hittable in self.hittables.iter() {
            let result = hittable.is_hit_by(
                ray,
                ray_t.with_max(winner.as_ref().map_or(ray_t.max, |hr| hr.value)),
            );

            if result.is_some() {
                winner = result;
            }
        }

        winner
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::color64::Color64;
    use crate::data::point64::Point64;
    use crate::hittables::sphere::Sphere;
    use crate::materials::Material;
    use crate::materials::diffuse_light::DiffuseLight;
    use approx::assert_abs_diff_eq;

    fn sphere_at_z(z: f64) -> Hittable {
        Hittable::Sphere(Sphere {
            center: Point64::new(0., 0., z),
            radius: 0.5,
            material: Material::DiffuseLight(DiffuseLight::new(Color64::new(1., 1., 1.))),
        })
    }

    #[test]
    fn closest_hit_wins_regardless_of_order() {
        let r = Ray {
            origin: Point64::new(0., 0., 0.),
            direction: Point64::new(0., 0., -1.),
            exposure_time: 0.,
        };
        let range = Interval::new(0.001, f64::INFINITY);
        for order in [[-5., -2.], [-2., -5.]] {
            let list = HittableVec {
                hittables: order.iter().map(|&z| sphere_at_z(z)).collect(),
            };
            let hit = list.is_hit_by(&r, range).unwrap();
            assert_abs_diff_eq!(hit.value, 1.5, epsilon = 1e-12);
        }
    }

    #[test]
    fn empty_list_misses() {
        let r = Ray {
            origin: Point64::new(0., 0., 0.),
            direction: Point64::new(0., 0., -1.),
            exposure_time: 0.,
        };
        let list = HittableVec { hittables: vec![] };
        assert!(list.is_hit_by(&r, Interval::UNIVERSE).is_none());
    }
}
