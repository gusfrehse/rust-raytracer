use crate::hittable::*;
use crate::utils;
use crate::vec3::*;

pub trait BSDF {
    fn emitted(&self, _wo: &Vec3, _info: &HitInfo) -> Option<Color> {
        None
    }

    fn eval(&self, wo: &Vec3, wi: &Vec3) -> Color;
    fn sample(&self, wo: &Vec3, info: HitInfo) -> (Vec3, Color, f64); // wi, f, pdf
    fn pdf(&self, wo: &Vec3, wi: &Vec3, info: HitInfo) -> f64;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl BSDF for Lambertian {
    fn eval(&self, _wo: &Vec3, _wi: &Vec3) -> Color {
        self.albedo / utils::PI
    }

    fn sample(&self, _wo: &Vec3, info: HitInfo) -> (Vec3, Color, f64) {
        let mut dir = info.normal + utils::random_unit_vector();

        if dir.is_zero() {
            dir = info.normal;
        }

        let wi = dir.unit();
        let pdf = wi.dot(info.normal).max(0.) / utils::PI;

        (wi, self.albedo / utils::PI, pdf)
    }

    fn pdf(&self, _wo: &Vec3, wi: &Vec3, info: HitInfo) -> f64 {
        wi.dot(info.normal).max(0.) / utils::PI
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl BSDF for Metal {
    //fn scatter(&self, r: &Ray, info: HitInfo) -> Option<(Ray, Color)> {
    //    let dir =
    //        utils::reflect(r.dir.unit(), info.normal) + self.fuzz * utils::random_point_in_sphere();

    //    if dir.dot(info.normal) <= 0.0 {
    //        None
    //    } else {
    //        let scattered = Ray { orig: info.p, dir };
    //        let attenuation = self.albedo;

    //        Some((scattered, attenuation))
    //    }
    //}

    fn eval(&self, _wo: &Vec3, _wi: &Vec3) -> Color {
        Vec3::new(0., 1., 0.) / utils::PI
    }

    fn sample(&self, _wo: &Vec3, info: HitInfo) -> (Vec3, Color, f64) {
        let mut dir = info.normal + utils::random_unit_vector();

        if dir.is_zero() {
            dir = info.normal;
        }

        let wi = dir.unit();
        let pdf = wi.dot(info.normal).max(0.);

        (wi, Vec3::new(0., 1., 0.) / utils::PI, pdf)
    }

    fn pdf(&self, _wo: &Vec3, wi: &Vec3, info: HitInfo) -> f64 {
        wi.dot(info.normal).max(0.) / utils::PI
    }
}

pub struct Dieletric {
    pub ir: f64,
}

impl Dieletric {
    //fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    //    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    //    let r0 = r0 * r0;
    //    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    //}
}

impl BSDF for Dieletric {
    //fn scatter(&self, r: &Ray, info: HitInfo) -> Option<(Ray, Color)> {
    //    let attenuation = Color::new(1.0, 1.0, 1.0);

    //    let refraction_ratio = if info.front_face {
    //        1.0 / self.ir
    //    } else {
    //        self.ir
    //    };

    //    let unit_direction = r.direction().unit();

    //    let cos_theta = info.normal.dot(-1.0 * unit_direction).min(1.0);
    //    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
    //    let cannot_refract = refraction_ratio * sin_theta > 1.0;
    //    let will_reflect = Self::reflectance(cos_theta, refraction_ratio) > utils::random_double();

    //    let direction = if cannot_refract || will_reflect {
    //        utils::reflect(unit_direction, info.normal)
    //    } else {
    //        utils::refract(unit_direction, info.normal, refraction_ratio)
    //    };

    //    let scattered = Ray {
    //        orig: info.p,
    //        dir: direction,
    //    };

    //    Some((scattered, attenuation))
    //}

    fn eval(&self, _wo: &Vec3, _wi: &Vec3) -> Color {
        Vec3::new(1.0, 0.0, 0.0) / utils::PI
    }

    fn sample(&self, _wo: &Vec3, info: HitInfo) -> (Vec3, Color, f64) {
        let mut dir = info.normal + utils::random_unit_vector();

        if dir.is_zero() {
            dir = info.normal;
        }

        let wi = dir.unit();
        let pdf = wi.dot(info.normal).max(0.);

        (wi, Vec3::new(1.0, 0.0, 0.0) / utils::PI, pdf)
    }

    fn pdf(&self, _wo: &Vec3, wi: &Vec3, info: HitInfo) -> f64 {
        wi.dot(info.normal).max(0.) / utils::PI
    }
}

pub struct DiffuseLight {
    pub intensity: Color,
}

impl BSDF for DiffuseLight {
    fn emitted(&self, _wo: &Vec3, info: &HitInfo) -> Option<Color> {
        if info.front_face {
            Some(self.intensity)
        } else {
            None
        }
    }

    fn eval(&self, _wo: &Vec3, _wi: &Vec3) -> Color {
        Color::zero()
    }

    fn sample(&self, _wo: &Vec3, _info: HitInfo) -> (Vec3, Color, f64) {
        unreachable!();
    }

    fn pdf(&self, _wo: &Vec3, _wi: &Vec3, _info: HitInfo) -> f64 {
        unreachable!();
    }
}
