use crate::ray::*;
use crate::vec3::*;

#[derive(Debug, Copy, Clone)]
pub struct HitInfo {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitInfo {
    pub fn new(r: &Ray, p: Vec3, outward_normal: Vec3, t: f64) -> HitInfo {
        let front_face = r.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -1.0 * outward_normal
        };

        HitInfo {
            p,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo>;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn add<T: Hittable + 'static>(&mut self, obj: T) {
        self.objects.push(Box::new(obj));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {
        let mut closest = t_max;
        let mut tmp_info: Option<HitInfo> = None;

        for o in &self.objects {
            if let Some(info) = o.hit(ray, t_min, closest) {
                tmp_info = Some(info);
                closest = info.t;
            }
        }

        return tmp_info;
    }
}
