/// A closed range `[min, max]` on the real line, used for ray-parameter bounds
/// (book 1, section 6.8).
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

#[allow(dead_code)]
impl Interval {
    /// Contains nothing: `min > max`.
    pub const EMPTY: Interval = Interval::new(f64::INFINITY, f64::NEG_INFINITY);
    /// Contains everything.
    pub const UNIVERSE: Interval = Interval::new(f64::NEG_INFINITY, f64::INFINITY);

    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    /// `min <= x <= max`
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    /// `min < x < max`
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        x.clamp(self.min, self.max)
    }

    /// Same lower bound, new upper bound (used to shrink the search as closer hits are found).
    pub fn with_max(&self, max: f64) -> Self {
        Self::new(self.min, max)
    }

    /// Same upper bound, new lower bound.
    pub fn with_min(&self, min: f64) -> Self {
        Self::new(min, self.max)
    }
}

#[cfg(test)]
mod test {
    use super::Interval;

    #[test]
    fn contains_vs_surrounds() {
        let i = Interval::new(0., 1.);
        assert!(i.contains(0.) && i.contains(1.) && i.contains(0.5));
        assert!(!i.surrounds(0.) && !i.surrounds(1.) && i.surrounds(0.5));
        assert!(!i.contains(-0.1) && !i.contains(1.1));
    }

    #[test]
    fn empty_and_universe() {
        assert!(!Interval::EMPTY.contains(0.));
        assert!(Interval::EMPTY.size() < 0.);
        assert!(Interval::UNIVERSE.surrounds(1e300) && Interval::UNIVERSE.surrounds(-1e300));
    }

    #[test]
    fn clamp_size_and_builders() {
        let i = Interval::new(-1., 3.);
        assert_eq!(i.size(), 4.);
        assert_eq!(i.clamp(-5.), -1.);
        assert_eq!(i.clamp(5.), 3.);
        assert_eq!(i.clamp(2.), 2.);
        assert_eq!(i.with_max(2.), Interval::new(-1., 2.));
        assert_eq!(i.with_min(0.), Interval::new(0., 3.));
    }
}
