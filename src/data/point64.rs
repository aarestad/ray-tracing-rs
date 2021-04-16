use nalgebra::Vector3;
use std::ops::Deref;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Point64(pub Vector3<f64>);

impl Point64 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Point64(Vector3::new(x, y, z))
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }

    pub fn z(&self) -> f64 {
        self.0[2]
    }
}

impl Deref for Point64 {
    type Target = Vector3<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
