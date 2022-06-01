use crate::ray::*;
use crate::vec3::*;
use crate::utils;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    vertical: Vec3,
    horizontal: Vec3,
}

impl Camera {
    pub fn new(vfov: f64, aspect_ratio: f64) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;
        let focal_length = 1.0;

        let origin = Point3::new(0, 0, 0);
        let horizontal = Vec3::new(viewport_width, 0, 0);
        let vertical = Vec3::new(0, viewport_height, 0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0, 0, focal_length);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            orig: self.origin,
            dir: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        }
    }
}

fn degrees_to_radians(deg: f64) -> f64 {
    utils::PI * deg / 180.0
}
