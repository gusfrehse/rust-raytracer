use crate::ray::*;
use crate::vec3::*;
use crate::hittable::*;
use crate::utils;

pub trait Material {
    fn scatter(&self, r: &Ray, info: HitInfo) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    pub albedo: Color
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, info: HitInfo) -> Option<(Ray, Color)> {
        let mut dir = info.normal + utils::random_unit_vector();

        if dir.is_zero() {
            dir = info.normal;
        }

        let scattered = Ray { orig: info.p,  dir };
        let attenuation = self.albedo;

        Some((scattered, attenuation))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, info: HitInfo) -> Option<(Ray, Color)> {
        let dir = utils::reflect(r.dir.unit(), info.normal) + self.fuzz * utils::random_point_in_sphere();

        if dir.dot(info.normal) <= 0.0 {
            None
        } else {
            let scattered = Ray { orig: info.p,  dir };
            let attenuation = self.albedo;

            Some((scattered, attenuation))
        }
    }
}

pub struct Dieletric {
    pub ir : f64,
}

impl Material for Dieletric {
    fn scatter(&self, r: &Ray, info: HitInfo) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);

        let refraction_ratio = if info.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let scattered = Ray {
            orig: info.p,
            dir: utils::refract(r.dir, info.normal, refraction_ratio),
        };

        Some((scattered, attenuation))
    }
}

