use crate::hittable::*;
use crate::utils;
use crate::vec3::*;

pub trait BRDF {
    fn is_delta(&self) -> bool {
        false
    }

    fn emitted(&self, _wo: &Vec3, _info: &HitInfo) -> Option<Color> {
        None
    }

    fn eval(&self, wo: &Vec3, wi: &Vec3, info: &HitInfo) -> Color;
    fn sample(&self, wo: &Vec3, info: HitInfo) -> (Vec3, Color, f64); // wi, f, pdf
    fn pdf(&self, wo: &Vec3, wi: &Vec3, info: HitInfo) -> f64;

    fn pl(&self, _prev_p: &Point3, _info: &HitInfo) -> Option<f64> {
        None
    }
}

pub struct Lambertian {
    pub albedo: Color,
}

impl BRDF for Lambertian {
    fn eval(&self, _wo: &Vec3, _wi: &Vec3, _info: &HitInfo) -> Color {
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

pub struct Phong {
    pub albedo: Color,
    pub kd: f64,
    pub ks: f64,
    pub shininess: f64,
}

impl BRDF for Phong {
    fn emitted(&self, _wo: &Vec3, _info: &HitInfo) -> Option<Color> {
        None
    }

    fn eval(&self, wo: &Vec3, wi: &Vec3, info: &HitInfo) -> Color {
        let n = info.normal;
        let r = utils::reflect(-*wo, n);
        let cos_alpha = r.dot(*wi).max(0.0);
        let spec =
            self.ks * (self.shininess + 2.0) / (2.0 * utils::PI) * cos_alpha.powf(self.shininess);
        self.albedo * (self.kd / utils::PI + spec)
    }

    fn sample(&self, wo: &Vec3, info: HitInfo) -> (Vec3, Color, f64) {
        let n = info.normal;
        let r = utils::reflect(-*wo, n);
        let w_spec = self.ks / (self.kd + self.ks);

        let wi: Vec3;
        let f: Color;

        if utils::random_double() < w_spec {
            // specular lobe: sample cos^n(alpha) around r
            let z = r;
            let up = if z.e[0].abs() > 0.9 {
                Vec3::new(0.0, 1.0, 0.0)
            } else {
                Vec3::new(1.0, 0.0, 0.0)
            };
            let x = up.cross(z).unit();
            let y = z.cross(x);

            let cos_alpha = utils::random_double().powf(1.0 / (self.shininess + 1.0));
            let sin_alpha = (1.0 - cos_alpha * cos_alpha).sqrt();
            let phi = 2.0 * utils::PI * utils::random_double();

            wi = sin_alpha * phi.cos() * x + sin_alpha * phi.sin() * y + cos_alpha * z;
            f = self.eval(wo, &wi, &info);
        } else {
            // diffuse lobe: cosine hemisphere (same as Lambertian)
            let mut dir = n + utils::random_unit_vector();
            if dir.is_zero() {
                dir = n;
            }
            wi = dir.unit();
            f = self.eval(wo, &wi, &info);
        }

        // always compute the full mixture pdf at wi
        let pdf_diff = wi.dot(n).max(0.0) / utils::PI;
        let cos_alpha = r.dot(wi).max(0.0);
        let pdf_spec = (self.shininess + 1.0) / (2.0 * utils::PI) * cos_alpha.powf(self.shininess);
        let pdf = (1.0 - w_spec) * pdf_diff + w_spec * pdf_spec;
        (wi, f, pdf)
    }

    fn pdf(&self, wo: &Vec3, wi: &Vec3, info: HitInfo) -> f64 {
        let n = info.normal;
        let r = utils::reflect(-*wo, n);
        let cos_alpha = r.dot(*wi).max(0.0);

        let pdf_diff = wi.dot(n).max(0.0) / utils::PI;
        let pdf_spec =
            (self.shininess + 1.0) / (2.0 * utils::PI) * cos_alpha.powf(self.shininess).max(0.0);

        let w_spec = self.ks / (self.kd + self.ks);
        (1.0 - w_spec) * pdf_diff + w_spec * pdf_spec
    }
}

pub struct DiffuseLight {
    pub intensity: Color,
    pub area: f64,
}

impl BRDF for DiffuseLight {
    fn emitted(&self, _wo: &Vec3, _info: &HitInfo) -> Option<Color> {
        Some(self.intensity)
    }

    fn eval(&self, _wo: &Vec3, _wi: &Vec3, _info: &HitInfo) -> Color {
        Color::zero()
    }

    fn sample(&self, _wo: &Vec3, _info: HitInfo) -> (Vec3, Color, f64) {
        unreachable!();
    }

    fn pdf(&self, _wo: &Vec3, _wi: &Vec3, _info: HitInfo) -> f64 {
        unreachable!();
    }

    fn pl(&self, prev_p: &Point3, info: &HitInfo) -> Option<f64> {
        let to_light = info.p - *prev_p;
        let d2 = to_light.length2();
        let cos_at_light = (-to_light.unit()).dot(info.normal).max(0.0).max(1e-8);
        Some((1.0 / self.area) * d2 / cos_at_light)
    }
}

pub struct Mirror {
    pub albedo: Color,
}

impl BRDF for Mirror {
    fn is_delta(&self) -> bool {
        true
    }

    fn eval(&self, _wo: &Vec3, _wi: &Vec3, _info: &HitInfo) -> Color {
        Color::zero()
    }

    fn sample(&self, wo: &Vec3, info: HitInfo) -> (Vec3, Color, f64) {
        let n = info.normal;
        let wi = utils::reflect(-*wo, n);
        let cos_theta = wi.dot(n).abs().max(1e-8);
        (wi, self.albedo / cos_theta, 1.0)
    }

    fn pdf(&self, _wo: &Vec3, _wi: &Vec3, _info: HitInfo) -> f64 {
        0.0
    }
}
