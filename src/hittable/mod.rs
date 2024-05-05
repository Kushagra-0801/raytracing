use std::ops::RangeInclusive;

use crate::{position::Position, ray::Ray};

mod sphere;
pub use sphere::Sphere;

pub trait Hittable {
    fn hit(&self, r: Ray, valid_t_range: RangeInclusive<f64>) -> Option<HitRecord>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FaceSide {
    Inward,
    Outward,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HitRecord {
    pub p: Position,
    pub n: Position,
    pub t: f64,
    pub face: FaceSide,
}

impl<T: Hittable> Hittable for Vec<T> {
    fn hit(&self, r: Ray, valid_t_range: RangeInclusive<f64>) -> Option<HitRecord> {
        let mut narrowed_range = valid_t_range;
        self.iter()
            .flat_map(|h| {
                h.hit(r, narrowed_range.clone()).inspect(|hr| {
                    let start = *narrowed_range.start();
                    let end = hr.t.min(*narrowed_range.end());
                    narrowed_range = start..=end;
                })
            })
            .last()
    }
}
