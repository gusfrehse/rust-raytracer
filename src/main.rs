use image::{ImageBuffer, RgbImage};
use std::rc::Rc;

pub mod camera;
pub mod hittable;
pub mod light_sampler;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod utils;
pub mod vec3;

use crate::camera::*;
use crate::hittable::*;
use crate::light_sampler::*;
use crate::material::*;
use crate::ray::*;
use crate::sphere::*;
use crate::utils::*;
use crate::vec3::*;

const RAY_MIN_T: f64 = 1e-10_f64;
const RAY_MAX_T: f64 = 1e10_f64;

fn main() {
    // image
    let aspect_ratio: f64 = 1.0;
    let output_width: u32 = 700;
    let output_height: u32 = (output_width as f64 / aspect_ratio).ceil() as u32;

    // world
    //let (world, lights) = random_scene();
    let (world, lights) = three_ball_scene();

    // camera
    let lookfrom = Point3::new(0, 4, 12);
    let lookat = Point3::new(0, 1, 0);
    let vup = Vec3::new(0, 1, 0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let fov = 65.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        fov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let samples_per_pixel = 20;

    let mut img: RgbImage = ImageBuffer::new(output_width, output_height);
    let mut variance_img: RgbImage = ImageBuffer::new(output_width, output_height);
    let mut mis_img: RgbImage = ImageBuffer::new(output_width, output_height);

    println!();

    for j in 0..output_height {
        for i in 0..output_width {
            let mut color = Color::zero();
            let mut variance = Color::zero();
            let mut mis_viz = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / (output_width as f64 - 1.0);
                let v = (j as f64 + random_double()) / (output_height as f64 - 1.0);

                let r = cam.get_ray(u, v);

                let (c, m) = li(&r, &world, &lights);
                color = color + c;
                variance = variance + c * c;
                mis_viz = mis_viz + m;
            }

            color = color / samples_per_pixel as f64;
            variance = variance / samples_per_pixel as f64 - color * color;
            mis_viz = mis_viz / samples_per_pixel as f64;

            // invert y axis
            write_pixel(&mut img, i, output_height - j - 1, color, true);
            write_pixel(&mut variance_img, i, output_height - j - 1, variance, false);
            write_pixel(&mut mis_img, i, output_height - j - 1, mis_viz, false);
            //print!("\r");
            print!(
                "{:5}/{:5}. {:.2}%",
                1 + output_width * j + i,
                output_height * output_width,
                (1 + output_width * j + i) as f64 * 100.0 / (output_height * output_width) as f64
            );
            print!("\n");
        }
    }
    println!();

    img.save("output.png").unwrap();
    variance_img.save("variance.png").unwrap();
    mis_img.save("mis_viz.png").unwrap();
}

const RR_MIN_BOUNCES: u64 = 3;
const RR_CLAMP: f64 = 0.95;
const MAX_DEPTH: u64 = 1000;

fn li(initial_ray: &Ray, world: &HittableList, lights: &LightSampler) -> (Color, Color) {
    let mut throughput = Color::new(1.0, 1.0, 1.0);
    let mut radiance = Color::new(0.0, 0.0, 0.0);
    let mut mis_viz = Color::zero();

    let mut depth = 0u64;
    let mut ray = initial_ray.clone();
    let mut prev_was_delta = false;
    let mut prev_p = Point3::zero();
    let mut prev_bsdf_pdf = 0.0f64;

    loop {
        if depth >= MAX_DEPTH {
            break;
        }

        if let Some(info) = world.hit(&ray, RAY_MIN_T, RAY_MAX_T) {
            // light hit (current hit is in an emissive material)
            if let Some(emitted) = info.material.emitted(&(-ray.dir), &info) {
                if depth == 0 {
                    radiance += emitted;
                    mis_viz += Color::new(0.0, emitted.max_component(), 0.0);
                } else if prev_was_delta {
                    let radiance_increment = throughput * emitted;
                    radiance += radiance_increment;
                    mis_viz += Color::new(0.0, radiance_increment.max_component(), 0.0);
                } else if let Some(pl_angle) = info.material.pl(&prev_p, &info) {
                    let w_bsdf =
                        prev_bsdf_pdf.powf(2.0) / (prev_bsdf_pdf.powf(2.0) + pl_angle.powf(2.0));

                    let radiance_increment = w_bsdf * throughput * emitted;

                    radiance += radiance_increment;
                    mis_viz += Color::new(0.0, 0.0, radiance_increment.max_component());
                }

                break;
            }

            // not in an emissive material, so now we sample a point in a light and cast a _shadow
            // ray_. if it hits the light (not occluded) we add this new path to the total radiance
            // count.
            if !info.material.is_delta() {
                let (light_pos, _light_material, pl) = lights.sample(); // select light
                let to_light = light_pos - info.p;
                let d2 = to_light.length2();
                let wi = to_light.unit();
                let wo = (-ray.dir).unit();

                let shadow_ray = Ray {
                    orig: info.p,
                    dir: wi,
                };

                if let Some(light_info) = world.hit(&shadow_ray, RAY_MIN_T, RAY_MAX_T) {
                    if let Some(emitted) = light_info.material.emitted(&shadow_ray.dir, &light_info)
                    {
                        let cos_theta_light = (-wi).dot(light_info.normal).max(0.0);
                        let cos_theta_bounce = wi.dot(info.normal).max(0.0);
                        let f = info.material.eval(&wo, &wi, &info);

                        let pb = info.material.pdf(&wo, &wi, info.clone());

                        let pl_angle = pl * d2 / cos_theta_light;
                        let w = pl_angle.powf(2.0) / (pb.powf(2.0) + pl_angle.powf(2.0));

                        let radiance_increment =
                            w * throughput * cos_theta_bounce * f * emitted / (pl_angle);

                        //radiance += radiance_increment;
                        //mis_viz += Color::new(radiance_increment.max_component(), 0.0, 0.0);
                    }
                }
            }

            // russian roulette
            if depth >= RR_MIN_BOUNCES {
                let q = throughput.max_component().min(RR_CLAMP).max(1e-8);
                if random_double() >= q {
                    break;
                }
                throughput = throughput / q;
            }

            let (sampled_dir, f, bsdf_pdf) = info.material.sample(&(-ray.dir).unit(), info.clone());
            prev_was_delta = info.material.is_delta();
            prev_bsdf_pdf = bsdf_pdf;
            prev_p = info.p;

            let cos_theta = sampled_dir.dot(info.normal).max(0.0);

            throughput = throughput * f * cos_theta / bsdf_pdf;

            ray.orig = info.p;
            ray.dir = sampled_dir;
            depth += 1;
        } else {
            //radiance += throughput * sky_color(ray.dir);
            break;
        }
    }

    (radiance, mis_viz)
}

fn write_pixel<U>(
    img: &mut image::ImageBuffer<image::Rgb<u8>, U>,
    x: u32,
    y: u32,
    c: Color,
    gamma_correction: bool,
) where
    U: std::ops::Deref<Target = [u8]> + std::ops::DerefMut, // rust is simple..
{
    if gamma_correction {
        let p = image::Rgb([
            (c.e[0].sqrt() * 255.999).floor() as u8,
            (c.e[1].sqrt() * 255.999).floor() as u8,
            (c.e[2].sqrt() * 255.999).floor() as u8,
        ]);

        img.put_pixel(x, y, p);
    } else {
        let p = image::Rgb([
            (c.e[0] * 255.999).floor() as u8,
            (c.e[1] * 255.999).floor() as u8,
            (c.e[2] * 255.999).floor() as u8,
        ]);
        img.put_pixel(x, y, p);
    }
}

fn random_scene() -> (HittableList, LightSampler) {
    let mut world = HittableList::new();
    let mut lights = LightSampler::new();

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

    let light = Rc::new(DiffuseLight {
        intensity: Color::new(525.0, 525.0, 525.0),
        area: 4.0 * PI * 0.5 * 0.5,
    });
    let light_sphere = Sphere::new(Point3::new(5, 10, 0), 0.5, light.clone());
    world.add(light_sphere.clone());
    lights.add(light_sphere.clone());

    (world, lights)
}

fn three_ball_scene() -> (HittableList, LightSampler) {
    let mut world = HittableList::new();
    let mut lights = LightSampler::new();

    // ground
    world.add(Sphere::new(
        Point3::new(0.0, -1000, 0.0),
        1000.0,
        Rc::new(Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        }),
    ));

    // left sphere — red Lambertian (diffuse)
    let red = Rc::new(Lambertian {
        albedo: Color::new(0.8, 0.1, 0.1),
    });

    // middle sphere — blue Phong (glossy specular)
    let blue = Rc::new(Phong {
        albedo: Color::new(0.1, 0.1, 0.8),
        kd: 0.3,
        ks: 0.7,
        shininess: 64.0,
    });

    // right sphere — greenish mirror (perfect specular)
    let mirror = Rc::new(Mirror {
        albedo: Color::new(0.2, 0.9, 0.2),
    });

    let r = 2.0;
    // spheres sit on the ground (center y = radius)
    world.add(Sphere::new(Point3::new(-4.5, r, 0.0), r, red));
    world.add(Sphere::new(Point3::new(0.0, r, 0.0), r, mirror));
    world.add(Sphere::new(Point3::new(4.5, r, 0.0), r, blue));

    // warm area light, off-center to cast visible shadows
    let light_sphere_radius = 2.0;
    let light = Rc::new(DiffuseLight {
        intensity: Color::new(10.0, 7.0, 2.0),
        area: 4.0 * PI * light_sphere_radius * light_sphere_radius,
    });
    let light_sphere = Sphere::new(Point3::new(-3.0, 10.0, -2.0), light_sphere_radius, light);

    world.add(light_sphere.clone());
    lights.add(light_sphere.clone());

    // ambient sky-like light (no NEE — just for BSDF sampling to hit)
    let ambient = Rc::new(DiffuseLight {
        intensity: Color::new(0.3, 0.25, 0.4),
        area: 4.0 * PI * 1000.0 * 1000.0,
    });
    world.add(Sphere::new(Point3::new(0.0, 0.0, 0.0), 1000.0, ambient));

    (world, lights)
}

fn power_heuristic(p1: f64, p2: f64, beta: f64) -> f64 {
    p1.powf(beta) / (p1.powf(beta) + p2.powf(beta))
}
