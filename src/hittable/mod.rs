use std::{fmt::Debug, rc::Rc};

use crate::{interval::Interval, material::Material, position::Position, ray::Ray};

mod sphere;
pub use sphere::Sphere;

pub trait Hittable {
    fn hit(&self, r: Ray, valid_t_range: Interval) -> Option<HitRecord>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FaceSide {
    Inward,
    Outward,
}

#[derive(Clone)]
pub struct HitRecord {
    pub incidence_point: Position,
    pub normal_vector: Position,
    pub t: f64,
    pub face: FaceSide,
    pub material: Rc<dyn Material>,
}

impl Debug for HitRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HitRecord")
            .field("incidence_point", &self.incidence_point)
            .field("normal_vector", &self.normal_vector)
            .field("t", &self.t)
            .field("face", &self.face)
            .finish()
    }
}

impl<T: Hittable> Hittable for Vec<T> {
    fn hit(&self, r: Ray, valid_t_range: Interval) -> Option<HitRecord> {
        let mut narrowed_range = valid_t_range;
        self.iter()
            .flat_map(|h| {
                h.hit(r, narrowed_range).inspect(|hr| {
                    let start = narrowed_range.start;
                    let end = hr.t.min(narrowed_range.end);
                    narrowed_range = Interval { start, end };
                })
            })
            .last()
    }
}

impl<T: Hittable> Hittable for &T {
    fn hit(&self, r: Ray, valid_t_range: Interval) -> Option<HitRecord> {
        (*self).hit(r, valid_t_range)
    }
}
