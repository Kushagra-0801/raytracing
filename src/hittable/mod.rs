use std::ops::RangeInclusive;

use crate::{position::Position, ray::Ray};

mod sphere;
pub use sphere::Sphere;

pub trait Hittable {
    fn hit(&self, r: Ray, valid_t_range: RangeInclusive<f64>) -> Option<HitRecord>;
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct HitRecord {
    pub p: Position,
    pub n: Position,
    pub t: f64,
}
