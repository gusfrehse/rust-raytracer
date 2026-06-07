use image::{ImageBuffer, RgbImage};
use std::rc::Rc;
use utils::random_double;

pub mod camera;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod utils;
pub mod vec3;

use crate::camera::*;
use crate::hittable::*;
use crate::material::*;
use crate::ray::*;
use crate::sphere::*;
use crate::vec3::*;

fn main() {
    // image
    let aspect_ratio: f64 = 1.0;
    let output_width: u32 = 250;
    let output_height: u32 = (output_width as f64 / aspect_ratio).ceil() as u32;

    // world
    let world = random_scene();

    // camera
    let lookfrom = Point3::new(13, 2, 3);
    let lookat = Point3::new(0, 0, 0);
    let vup = Vec3::new(0, 1, 0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let fov = 20.0;
    let aspect_ratio = 16.0 / 9.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        fov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let samples_per_pixel = 3;
    let max_depth = 3;

    let mut img: RgbImage = ImageBuffer::new(output_width, output_height);

    println!();

    for j in 0..output_height {
        for i in 0..output_width {
            let mut color = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / (output_width as f64 - 1.0);
                let v = (j as f64 + random_double()) / (output_height as f64 - 1.0);

                let r = cam.get_ray(u, v);

                color = color + li(&r, &world, max_depth);
            }

            color = color / samples_per_pixel as f64;

            // invert y axis
            write_pixel(&mut img, i, output_height - j - 1, color);
            print!("\r");
            print!(
                "{:5}/{:5}. {:.2}%",
                1 + output_width * j + i,
                output_height * output_width,
                (1 + output_width * j + i) as f64 * 100.0 / (output_height * output_width) as f64
            );
        }
    }
    println!();

    img.save("test.png").unwrap();
}

fn li(initial_ray: &Ray, world: &HittableList, initial_depth: u64) -> Color {
    let mut throughput = Color::new(1.0, 1.0, 1.0);
    let radiance;

    let mut depth = initial_depth;
    let mut ray = initial_ray.clone();

    loop {
        if depth == 0 {
            return Color::zero();
        }

        if let Some(info) = world.hit(&ray, 1e-10_f64, 1e10_f64) {
            let (sampled_dir, f, pdf) = info.material.sample(&ray.dir, info.clone());

            let cos_theta = sampled_dir.dot(info.normal).max(0.0);

            throughput = throughput * (f * cos_theta / pdf);

            ray.orig = info.p;
            ray.dir = sampled_dir;
            depth = depth - 1;
        } else {
            break;
        }
    }

    let unit_dir = ray.direction().unit();
    let t = 0.5 * (unit_dir.e[1] + 1.0);

    radiance = throughput * ((1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0));

    radiance
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

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    world.add(Sphere::new(
        Point3::new(0.0, -1000, 0.0),
        1000.0,
        Rc::new(Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        }),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();

            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + random_double(),
            );
            if (center - Point3::new(4, 0.2, 0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::new(random_double(), random_double(), random_double());
                    let sphere_mat = Rc::new(Lambertian { albedo });
                    world.add(Sphere::new(center, 0.2, sphere_mat));
                } else if choose_mat < 0.95 {
                    let albedo = Color::new(random_double(), random_double(), random_double());
                    let fuzz = random_double();
                    let sphere_mat = Rc::new(Metal { albedo, fuzz });
                    world.add(Sphere::new(center, 0.2, sphere_mat));
                } else {
                    let sphere_mat = Rc::new(Dieletric { ir: 1.5 });
                    world.add(Sphere::new(center, 0.2, sphere_mat));
                }
            }
        }
    }

    let glass = Rc::new(Dieletric { ir: 1.5 });

    let red = Rc::new(Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    });

    let metal = Rc::new(Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });

    world.add(Sphere::new(Point3::new(0, 1, 0), 1.0, glass.clone()));

    world.add(Sphere::new(Point3::new(-4, 1, 0), 1.0, red.clone()));
    world.add(Sphere::new(Point3::new(4, 1, 0), 1.0, metal.clone()));

    world
}
