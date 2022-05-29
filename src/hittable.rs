
use crate::vec3::*;
use crate::ray::*;

#[derive(Debug, Copy, Clone)]
pub struct HitInfo {
    pub p : Point3,
    pub normal : Vec3,
    pub t : f64,
}

pub trait Hittable {
    fn hit(&self, ray : &Ray, t_min : f64, t_max : f64) -> Option<HitInfo>;
}
