use image::{ImageBuffer, RgbImage};
use rand::prelude::*;

pub mod camera;
pub mod hittable;
pub mod ray;
pub mod sphere;
pub mod vec3;
pub mod material;
pub mod utils;

use crate::camera::*;
use crate::hittable::*;
use crate::ray::*;
use crate::sphere::*;
use crate::vec3::*;
use crate::material::*;

fn main() {
    // image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let output_width: u32 = 720;
    let output_height: u32 = (output_width as f64 / aspect_ratio).floor() as u32;

    // world
    let mut world = HittableList::new();
    let blue = std::rc::Rc::new(Lambertian { albedo: Color::new(0.2, 0.1, 0.8) });
    let green = std::rc::Rc::new(Lambertian { albedo: Color::new(0.1, 0.8, 0.1) });
    let metal = std::rc::Rc::new(Metal { albedo: Color::new(0.8, 0.8, 0.8), fuzz: 0.0});

    world.add(Sphere::new(Point3::new(0, 0, -1.2), 0.5, metal.clone()));
    world.add(Sphere::new(Point3::new(1.0, -0.1, -0.9), 0.4, green.clone()));
    world.add(Sphere::new(Point3::new(0, -100.5, -1), 100.0, blue.clone()));

    // camera
    let cam = Camera::new(90.0, 16.0 / 9.0);
    let samples_per_pixel = 50;
    let max_depth = 10;

    let mut img: RgbImage = ImageBuffer::new(output_width, output_height);

    let mut rng = rand::thread_rng();

    for i in 0..output_width {
        for j in 0..output_height {
            let mut color = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen_range(0.0..1.0)) / (output_width as f64 - 1.0);
                let v = (j as f64 + rng.gen_range(0.0..1.0)) / (output_height as f64 - 1.0);

                let r = cam.get_ray(u, v);

                color = color + ray_color(&r, &world, max_depth);
            }

            color = color / samples_per_pixel as f64;

            // invert y axis
            write_pixel(&mut img, i, output_height - j - 1, color);
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
        let target = info.p + info.normal + utils::random_unit_vector();
        col = 0.5
            * ray_color(
                &Ray {
                    orig: info.p,
                    dir: target - info.p,
                },
                world,
                depth - 1,
            );

        if let Some((scattered, atteunuation)) = info.material.scatter(ray, info.clone()) {
            col = atteunuation * ray_color(&scattered, world, depth - 1);
        }
    }

    col
}

fn write_pixel<U>(img: &mut image::ImageBuffer<image::Rgb<u8>, U>, x: u32, y: u32, c: Color)
where
    U: std::ops::Deref<Target = [u8]> + std::ops::DerefMut, // rust is simple..
{
    let p = image::Rgb([
        (c.e[0].sqrt() * 255.999).floor() as u8,
        (c.e[1].sqrt() * 255.999).floor() as u8,
        (c.e[2].sqrt() * 255.999).floor() as u8,
    ]);
    img.put_pixel(x, y, p);
}

