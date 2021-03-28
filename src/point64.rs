use crate::vec3_64::Vec3_64;
use std::ops::Deref;

pub struct Point64(pub Vec3_64);

impl Point64 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point64(Vec3_64(x, y, z))
    }

    pub fn _x(&self) -> &f64 {
        &self.0 .0
    }

    pub fn y(&self) -> &f64 {
        &self.0 .1
    }

    pub fn _z(&self) -> &f64 {
        &self.0 .2
    }
}

impl Deref for Point64 {
    type Target = Vec3_64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
