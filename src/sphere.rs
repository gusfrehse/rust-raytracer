use crate::hittable::*;
use crate::material::*;
use crate::ray::*;
use crate::utils::*;
use crate::vec3::*;

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: std::rc::Rc<dyn BSDF>,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64, mat: std::rc::Rc<dyn BSDF>) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            material: mat,
        }
    }

    pub fn sample_point(&self) -> (Point3, f64) {
        let dir = random_unit_vector();
        let p = self.center + self.radius * dir;
        let pdf = 1.0 / (4.0 * PI * self.radius * self.radius);
        (p, pdf)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {
        let orig_center = ray.orig - self.center;

        let a = ray.dir.length2();
        let h = orig_center.dot(ray.dir);
        let c = orig_center.length2() - self.radius * self.radius;

        let disc = h * h - a * c;

        if disc < 0.0 {
            return None;
        }

        let mut t = (-h - disc.sqrt()) / a;

        if t < t_min || t > t_max {
            t = (-h + disc.sqrt()) / a;

            if t < t_min || t > t_max {
                return None;
            }
        }

        let p = ray.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let info = HitInfo::new(ray, p, outward_normal, t, self.material.clone());

        Some(info)
    }
}
