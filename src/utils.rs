use rand::prelude::*;

use crate::vec3::*;

pub const PI: f64 = std::f64::consts::PI;
pub const INFINITY: f64 = std::f64::INFINITY;

pub fn random_double() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn random_unit_vector() -> Vec3 {
    random_point_in_sphere().unit()
}

pub fn random_point_in_sphere() -> Point3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Point3::new(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
        );

        if p.length2() < 1.0 {
            return p;
        }
    }
}

pub fn random_point_in_disk() -> Point3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Point3::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), 0.0);

        if p.length2() < 1.0 {
            return p;
        }
    }
}
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

pub fn refract(v: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = n.dot(-1.0 * v).min(1.0);
    let out_perp = etai_over_etat * (v + cos_theta * n);
    let out_parallel = -(1.0 - out_perp.length2()).sqrt() * n;

    out_perp + out_parallel
}
