use crate::data::vec3_64::Vec3_64;
use std::ops::Deref;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Point64(pub Vec3_64);

#[allow(dead_code)] // x() and z()
impl Point64 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Point64(Vec3_64(x, y, z))
    }

    pub fn x(&self) -> f64 {
        self.0 .0
    }

    pub fn y(&self) -> f64 {
        self.0 .1
    }

    pub fn z(&self) -> f64 {
        self.0 .2
    }
}

impl Deref for Point64 {
    type Target = Vec3_64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
