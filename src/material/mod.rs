use crate::{color::Color, hittable::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(&self, incident_ray: Ray, hit_record: HitRecord) -> Option<(Color, Ray)>;
}
