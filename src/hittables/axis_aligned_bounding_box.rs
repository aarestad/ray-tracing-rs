use crate::data::interval::Interval;
use crate::data::point64::Point64;
use crate::data::ray::Ray;
use std::mem;

#[derive(PartialEq, Copy, Clone)]
pub struct AxisAlignedBoundingBox {
    pub minimum: Point64,
    pub maximum: Point64,
}

/// Minimum thickness of a box along any axis (book 2, `aabb::pad_to_minimums`), so flat
/// primitives such as axis-aligned quads and triangles don't produce zero-width boxes.
const MIN_THICKNESS: f64 = 0.0001;

impl AxisAlignedBoundingBox {
    /// Box spanning two arbitrary corner points (in any order).
    pub fn from_corners(a: Point64, b: Point64) -> Self {
        AxisAlignedBoundingBox {
            minimum: Point64(a.0.inf(&b.0)),
            maximum: Point64(a.0.sup(&b.0)),
        }
    }

    /// Widens any axis thinner than `MIN_THICKNESS` to exactly that thickness, keeping it centered.
    pub fn padded(mut self) -> Self {
        for i in 0..3 {
            if self.maximum.0[i] - self.minimum.0[i] < MIN_THICKNESS {
                let mid = 0.5 * (self.minimum.0[i] + self.maximum.0[i]);
                self.minimum.0[i] = mid - MIN_THICKNESS / 2.;
                self.maximum.0[i] = mid + MIN_THICKNESS / 2.;
            }
        }
        self
    }

    /// Index (0 = x, 1 = y, 2 = z) of the box's longest side (book 2, section 3.10).
    pub fn longest_axis(&self) -> usize {
        let size = self.maximum.0 - self.minimum.0;
        if size.x > size.y {
            if size.x > size.z { 0 } else { 2 }
        } else if size.y > size.z {
            1
        } else {
            2
        }
    }

    pub fn is_hit_by(&self, ray: &Ray, ray_t: Interval) -> bool {
        self.hit_interval(ray, ray_t).is_some()
    }

    /// Ray segment `ray_t` clipped to this box (slab method, book 2 section 3.4). Returns the
    /// `[t_enter, t_exit]` sub-interval along the ray, or `None` on a miss.
    ///
    /// A zero direction component gives `±∞` slab bounds, which work as-is. If the origin also
    /// lies exactly on that slab's plane, `0 · ∞ = NaN`; `f64::max`/`min` ignore NaN, so that
    /// axis then places no constraint (the ray is treated as inside the slab).
    pub fn hit_interval(&self, ray: &Ray, ray_t: Interval) -> Option<Interval> {
        let mut t0 = ray_t.min;
        let mut t1 = ray_t.max;
        for idx in 0..3 {
            let inv_direction = 1. / ray.direction.0[idx];
            let mut ta = (self.minimum.0[idx] - ray.origin.0[idx]) * inv_direction;
            let mut tb = (self.maximum.0[idx] - ray.origin.0[idx]) * inv_direction;

            if inv_direction < 0. {
                mem::swap(&mut ta, &mut tb);
            }

            t0 = t0.max(ta);
            t1 = t1.min(tb);
            if t0 >= t1 {
                return None;
            }
        }
        Some(Interval::new(t0, t1))
    }

    pub fn surrounding_box_with(self, other: &AxisAlignedBoundingBox) -> AxisAlignedBoundingBox {
        if self == *other {
            return self;
        };

        let minimum = Point64::new(
            self.minimum.x().min(other.minimum.x()),
            self.minimum.y().min(other.minimum.y()),
            self.minimum.z().min(other.minimum.z()),
        );

        let maximum = Point64::new(
            self.maximum.x().max(other.maximum.x()),
            self.maximum.y().max(other.maximum.y()),
            self.maximum.z().max(other.maximum.z()),
        );

        AxisAlignedBoundingBox { minimum, maximum }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn unit_box() -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::from_corners(Point64::new(1., 1., 1.), Point64::new(0., 0., 0.))
    }

    fn ray(o: (f64, f64, f64), d: (f64, f64, f64)) -> Ray {
        Ray {
            origin: Point64::new(o.0, o.1, o.2),
            direction: Point64::new(d.0, d.1, d.2),
            exposure_time: 0.,
        }
    }

    const ALL: Interval = Interval::new(0.001, f64::INFINITY);

    #[test]
    fn from_corners_orders_points() {
        let b = unit_box();
        assert_eq!(b.minimum, Point64::new(0., 0., 0.));
        assert_eq!(b.maximum, Point64::new(1., 1., 1.));
    }

    #[test]
    fn hit_from_outside_reports_entry_and_exit() {
        let i = unit_box()
            .hit_interval(&ray((0.5, 0.5, -1.), (0., 0., 1.)), ALL)
            .unwrap();
        assert_eq!((i.min, i.max), (1., 2.));
    }

    #[test]
    fn origin_inside_box_hits() {
        let i = unit_box()
            .hit_interval(&ray((0.5, 0.5, 0.5), (1., 1., 1.)), ALL)
            .unwrap();
        assert_eq!((i.min, i.max), (0.001, 0.5));
    }

    #[test]
    fn misses() {
        let b = unit_box();
        assert!(!b.is_hit_by(&ray((2., 2., -1.), (0., 0., 1.)), ALL)); // beside the box
        assert!(!b.is_hit_by(&ray((0.5, 0.5, -1.), (0., 0., -1.)), ALL)); // pointing away
        assert!(!b.is_hit_by(
            &ray((0.5, 0.5, -1.), (0., 0., 1.)),
            Interval::new(0.001, 0.5)
        ));
    }

    #[test]
    fn axis_parallel_rays() {
        let b = unit_box();
        // Zero x/y direction components, origin strictly inside those slabs.
        assert!(b.is_hit_by(&ray((0.5, 0.5, -1.), (0., 0., 1.)), ALL));
        // Zero x component, origin outside the x slab: must miss.
        assert!(!b.is_hit_by(&ray((1.5, 0.5, -1.), (0., 0., 1.)), ALL));
        // Origin exactly on the x = 0 face plane with zero x direction (NaN case): hits.
        assert!(b.is_hit_by(&ray((0., 0.5, -1.), (0., 0., 1.)), ALL));
        // Negative zero direction component behaves like positive zero.
        assert!(b.is_hit_by(&ray((0.5, 0.5, -1.), (-0., 0., 1.)), ALL));
    }

    #[test]
    fn padding_only_widens_thin_axes() {
        let flat = AxisAlignedBoundingBox::from_corners(
            Point64::new(0., 2., 0.),
            Point64::new(1., 2., 1.),
        )
        .padded();
        assert!(flat.maximum.y() - flat.minimum.y() >= MIN_THICKNESS * 0.999);
        assert!((flat.minimum.y() + flat.maximum.y()) / 2. - 2. < 1e-12);
        assert_eq!((flat.minimum.x(), flat.maximum.x()), (0., 1.));
        // A flat box is hittable edge-on along its thin axis.
        assert!(flat.is_hit_by(&ray((0.5, 0., 0.5), (0., 1., 0.)), ALL));
    }

    #[test]
    fn longest_axis() {
        let b = |x, y, z| {
            AxisAlignedBoundingBox::from_corners(Point64::new(0., 0., 0.), Point64::new(x, y, z))
        };
        assert_eq!(b(3., 1., 2.).longest_axis(), 0);
        assert_eq!(b(1., 3., 2.).longest_axis(), 1);
        assert_eq!(b(1., 2., 3.).longest_axis(), 2);
    }
}
