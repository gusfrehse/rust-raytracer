use crate::hittable::*;
use crate::material::*;
use crate::ray::*;
use crate::vec3::*;

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: std::rc::Rc<dyn Material>,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64, mat: std::rc::Rc<dyn Material>) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            material: mat,
        }
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
