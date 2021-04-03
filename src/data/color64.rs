use crate::data::vec3_64::Vec3_64;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color64(pub Vec3_64);

impl Color64 {
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Color64(Vec3_64(r, g, b))
    }

    pub fn r(&self) -> f64 {
        self.0 .0
    }

    pub fn g(&self) -> f64 {
        self.0 .1
    }

    pub fn b(&self) -> f64 {
        self.0 .2
    }
}

impl Deref for Color64 {
    type Target = Vec3_64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Color64 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}