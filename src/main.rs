use image::{ImageBuffer, RgbImage};
use rand::prelude::*;

pub mod hittable;
pub mod ray;
pub mod sphere;
pub mod vec3;

use crate::hittable::*;
use crate::ray::*;
use crate::sphere::*;
use crate::vec3::*;

fn main() {
    // image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let width: u32 = 400;
    let height: u32 = (width as f64 / aspect_ratio).floor() as u32;

    // world
    let mut world = HittableList {
        objects: Vec::new(),
    };
    world.add(Sphere::new(Point3::new(0, 0, -1), 0.5));
    world.add(Sphere::new(Point3::new(0, -100.5, -1), 100.0));
    world.add(Sphere::new(Point3::new(2.0, -0.2, -1.5), 0.4));

    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let origin = Vec3::new(0, 0, 0);
    let horizontal = Vec3::new(viewport_width, 0, 0);
    let vertical = Vec3::new(0, viewport_height, 0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0, 0, focal_length);

    let mut img: RgbImage = ImageBuffer::new(width, height);

    let mut rng = rand::thread_rng();

    for i in 0..width {
        for j in 0..height {
            let mut color = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen_range(0.0..1.0)) / (width as f64 - 1.0);
                let v = (j as f64 + rng.gen_range(0.0..1.0)) / (height as f64 - 1.0);

                let r = Ray {
                    orig: origin,
                    dir: lower_left_corner + u * horizontal + v * vertical - origin,
                };

                color = color + ray_color(&r, &world, max_depth);
            }

            color = color / samples_per_pixel as f64;

            // invert y axis
            write_pixel(&mut img, i, height - j - 1, color);
        }
    }

    img.save("test.png").unwrap();
}

fn ray_color(ray: &Ray, world: &HittableList, depth: u64) -> Color {
    if depth == 0 {
        return Color::zero();
    }

    let unit_dir = ray.direction().unit();
    let t = 0.5 * (unit_dir.e[1] + 1.0);

    let mut col = (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);

    if let Some(info) = world.hit(ray, 0.0, 1000.0) {
        let target = info.p + info.normal + random_point_in_sphere();
        col = 0.5 * ray_color(&Ray { orig: info.p, dir: target - info.p }, world, depth - 1);
        }

    col
}

fn write_pixel<U>(img: &mut image::ImageBuffer<image::Rgb<u8>, U>, x: u32, y: u32, c: Color)
where
    U: std::ops::Deref<Target = [u8]> + std::ops::DerefMut, // rust is simple..
{
    let p = image::Rgb([
        (c.e[0] * 255.999).floor() as u8,
        (c.e[1] * 255.999).floor() as u8,
        (c.e[2] * 255.999).floor() as u8,
    ]);
    img.put_pixel(x, y, p);
}

fn random_point_in_sphere() -> Point3 {
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
