use rand::prelude::*;

use crate::vec3::*;

pub const PI : f64 = std::f64::consts::PI;
pub const INFINITY : f64 = std::f64::INFINITY;

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

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}
