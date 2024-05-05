use crate::{position::Position, ray::Ray, util::RangeSurround};

use super::{FaceSide, HitRecord, Hittable};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Sphere {
    center: Position,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Position, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, valid_t_range: std::ops::RangeInclusive<f64>) -> Option<HitRecord> {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = r.direction().dot(oc);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = h.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let d = discriminant.sqrt();
        let root1 = (h - d) / a;
        let root2 = (h + d) / a;
        if valid_t_range.surrounds(&root1) {
            let out_normal = (r.at(root1) - self.center) / self.radius;
            let (face, opp_normal) = if out_normal.dot(r.direction()) < 0.0 {
                (FaceSide::Outward, out_normal)
            } else {
                (FaceSide::Inward, -out_normal)
            };
            Some(HitRecord {
                p: r.at(root1),
                n: opp_normal,
                t: root1,
                face,
            })
        } else if valid_t_range.surrounds(&root2) {
            let out_normal = (r.at(root2) - self.center) / self.radius;
            let (face, opp_normal) = if out_normal.dot(r.direction()) < 0.0 {
                (FaceSide::Outward, out_normal)
            } else {
                (FaceSide::Inward, -out_normal)
            };
            Some(HitRecord {
                p: r.at(root2),
                n: opp_normal,
                t: root2,
                face,
            })
        } else {
            None
        }
    }
}
